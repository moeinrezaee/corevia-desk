#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use librustdesk::*;

#[cfg(any(target_os = "android", target_os = "ios", feature = "flutter"))]
fn main() {
    if !common::global_init() {
        eprintln!("Global initialization failed.");
        return;
    }
    common::test_rendezvous_server();
    common::test_nat_type();
    common::global_clean();
}

#[cfg(not(any(target_os = "android", target_os = "ios", feature = "flutter")))]
fn main() {
    #[cfg(all(windows, not(feature = "inline")))]
    unsafe {
        winapi::um::shellscalingapi::SetProcessDpiAwareness(2);
    }

    let instance_guard = if crate::single_instance::is_launcher_invocation() {
        match crate::single_instance::acquire() {
            Ok(crate::single_instance::AcquireResult::Primary(guard)) => Some(guard),
            Ok(crate::single_instance::AcquireResult::AlreadyRunning) => return,
            Err(error) => {
                eprintln!("Could not enforce single-instance launch: {error}");
                return;
            }
        }
    } else {
        None
    };

    if let Some(args) = crate::core_main::core_main().as_mut() {
        webview::start(args, instance_guard);
    }
    common::global_clean();
}
