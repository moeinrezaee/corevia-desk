use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    sync::mpsc::{self, TryRecvError},
    thread,
    time::Duration,
};

use tao::event_loop::EventLoopProxy;

const ACTIVATION_MESSAGE: &[u8] = b"corevia-desk-activate-v1";
const STARTING: &[u8] = b"starting";

pub enum AcquireResult {
    Primary(InstanceGuard),
    AlreadyRunning,
}

pub fn is_launcher_invocation() -> bool {
    is_launcher_arguments(std::env::args_os().skip(1))
}

fn is_launcher_arguments(arguments: impl IntoIterator<Item = std::ffi::OsString>) -> bool {
    arguments.into_iter().all(|argument| {
        let argument = argument.to_string_lossy();
        if argument.is_empty() {
            return true;
        }

        #[cfg(target_os = "macos")]
        {
            argument.starts_with("OSLogRateLimit=") || argument.starts_with("-psn_")
        }

        #[cfg(not(target_os = "macos"))]
        {
            false
        }
    })
}

pub struct InstanceGuard {
    listener: TcpListener,
    _lock_file: File,
    _platform_lock: PlatformLock,
    stop_sender: Option<mpsc::Sender<()>>,
    listener_thread: Option<thread::JoinHandle<()>>,
}

#[cfg(unix)]
enum PlatformLock {
    File,
}

#[cfg(windows)]
enum PlatformLock {
    Mutex(winapi::um::winnt::HANDLE),
}

pub fn acquire() -> io::Result<AcquireResult> {
    let path = instance_file_path();
    let lock_file = open_lock_file(&path)?;
    let platform_lock = match acquire_platform_lock(&lock_file)? {
        Some(lock) => lock,
        None => {
            activate_existing(&path);
            return Ok(AcquireResult::AlreadyRunning);
        }
    };

    write_endpoint(&lock_file, STARTING)?;
    let listener = TcpListener::bind(("127.0.0.1", 0))?;
    let port = listener.local_addr()?.port().to_string();
    write_endpoint(&lock_file, port.as_bytes())?;

    Ok(AcquireResult::Primary(InstanceGuard {
        listener,
        _lock_file: lock_file,
        _platform_lock: platform_lock,
        stop_sender: None,
        listener_thread: None,
    }))
}

impl InstanceGuard {
    pub fn start_listener<E>(
        &mut self,
        proxy: EventLoopProxy<E>,
        activation_event: E,
    ) -> io::Result<()>
    where
        E: Clone + Send + 'static,
    {
        if self.listener_thread.is_some() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "single-instance activation listener already started",
            ));
        }

        let listener = self.listener.try_clone()?;
        listener.set_nonblocking(true)?;
        let (stop_sender, stop_receiver) = mpsc::channel();

        let listener_thread = thread::Builder::new()
            .name("corevia-desk-activation".to_owned())
            .spawn(move || loop {
                if !matches!(stop_receiver.try_recv(), Err(TryRecvError::Empty)) {
                    break;
                }

                let (mut stream, _) = match listener.accept() {
                    Ok(connection) => connection,
                    Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(50));
                        continue;
                    }
                    Err(error) => {
                        hbb_common::log::warn!(
                            "Single-instance activation listener stopped: {error}"
                        );
                        break;
                    }
                };

                if let Err(error) = stream.set_read_timeout(Some(Duration::from_millis(250))) {
                    hbb_common::log::warn!("Could not set activation message timeout: {error}");
                    continue;
                }

                let mut message = [0; ACTIVATION_MESSAGE.len()];
                if stream.read_exact(&mut message).is_ok() && message == ACTIVATION_MESSAGE {
                    let _ = proxy.send_event(activation_event.clone());
                }
            })?;
        self.stop_sender = Some(stop_sender);
        self.listener_thread = Some(listener_thread);
        Ok(())
    }
}

impl Drop for InstanceGuard {
    fn drop(&mut self) {
        if let Some(stop_sender) = self.stop_sender.take() {
            let _ = stop_sender.send(());
        }
        if let Some(listener_thread) = self.listener_thread.take() {
            if listener_thread.join().is_err() {
                hbb_common::log::warn!("Single-instance activation listener panicked");
            }
        }
    }
}

fn instance_file_path() -> PathBuf {
    #[cfg(target_os = "linux")]
    let root = std::env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .filter(|path| path.is_dir())
        .unwrap_or_else(std::env::temp_dir);

    #[cfg(not(target_os = "linux"))]
    let root = std::env::temp_dir();

    #[cfg(unix)]
    let identity = format!("{}.{}", crate::branding::APP_BUNDLE_IDENTIFIER, unsafe {
        hbb_common::libc::getuid()
    });
    #[cfg(windows)]
    let identity = crate::branding::APP_BUNDLE_IDENTIFIER.to_owned();

    root.join(format!("{identity}.single-instance"))
}

fn open_lock_file(path: &PathBuf) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true);

    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }

    options.open(path)
}

fn write_endpoint(file: &File, bytes: &[u8]) -> io::Result<()> {
    file.set_len(0)?;
    let mut file = file;
    file.seek(SeekFrom::Start(0))?;
    file.write_all(bytes)?;
    file.flush()
}

fn activate_existing(path: &PathBuf) {
    for _ in 0..40 {
        if let Ok(mut file) = File::open(path) {
            let mut endpoint = String::new();
            if file.read_to_string(&mut endpoint).is_ok() {
                if let Ok(port) = endpoint.parse::<u16>() {
                    if let Ok(mut stream) = TcpStream::connect_timeout(
                        &([127, 0, 0, 1], port).into(),
                        Duration::from_millis(100),
                    ) {
                        let _ = stream.write_all(ACTIVATION_MESSAGE);
                        return;
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
}

#[cfg(unix)]
fn acquire_platform_lock(_file: &File) -> io::Result<Option<PlatformLock>> {
    use std::os::fd::AsRawFd;

    let result = unsafe {
        hbb_common::libc::flock(
            _file.as_raw_fd(),
            hbb_common::libc::LOCK_EX | hbb_common::libc::LOCK_NB,
        )
    };
    if result == 0 {
        return Ok(Some(PlatformLock::File));
    }

    let error = io::Error::last_os_error();
    let code = error.raw_os_error();
    if code == Some(hbb_common::libc::EWOULDBLOCK) || code == Some(hbb_common::libc::EAGAIN) {
        Ok(None)
    } else {
        Err(error)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn recognizes_empty_launcher_argument() {
        assert!(super::is_launcher_arguments([std::ffi::OsString::from("")]));
    }

    #[test]
    fn keeps_helper_arguments_out_of_the_gui_single_instance_gate() {
        assert!(!super::is_launcher_arguments([std::ffi::OsString::from(
            "--service"
        )]));
    }
}

#[cfg(windows)]
fn acquire_platform_lock(_file: &File) -> io::Result<Option<PlatformLock>> {
    use std::ptr::null_mut;
    use winapi::shared::winerror::ERROR_ALREADY_EXISTS;
    use winapi::um::{
        errhandlingapi::{GetLastError, SetLastError},
        handleapi::CloseHandle,
        synchapi::CreateMutexW,
    };

    let name = crate::platform::wide_string(&format!(
        "Local\\{}_instance",
        crate::branding::APP_BUNDLE_IDENTIFIER
    ));
    unsafe {
        SetLastError(0);
        let handle = CreateMutexW(null_mut(), 0, name.as_ptr());
        if handle.is_null() {
            return Err(io::Error::last_os_error());
        }
        if GetLastError() == ERROR_ALREADY_EXISTS {
            CloseHandle(handle);
            return Ok(None);
        }
        Ok(Some(PlatformLock::Mutex(handle)))
    }
}

impl Drop for PlatformLock {
    fn drop(&mut self) {
        #[cfg(windows)]
        match self {
            PlatformLock::Mutex(handle) => unsafe {
                winapi::um::handleapi::CloseHandle(*handle);
            },
        }
    }
}
