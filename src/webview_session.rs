//! Outgoing-session adapter for the active Nuxt/WebView desktop path.
//!
//! The original desktop UI starts an outgoing session in a separate process
//! and lets the Flutter/Sciter connection window own the Rust session.  The
//! active Corevia Desk path has no such legacy window, so the WebView host
//! needs a small native owner for the Rust connection loop.  This module owns
//! the transport/session lifecycle and exposes a safe snapshot to the bridge.
//! It deliberately does not pull Flutter, Dart, or Sciter into the release
//! path.

use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use hbb_common::{
    message_proto::{
        option_message::BoolOption, CursorData, CursorPosition, DisplayInfo, FileEntry, Misc,
        Message, OptionMessage, PeerInfo, SwitchDisplay, TerminalResponse, WindowsSession,
    },
    rendezvous_proto::ConnType,
};
use scrap::ImageFormat;
use serde_json::{json, Value};

use crate::{
    client::{Client, Data, FileManager, Interface, QualityStatus},
    ui_session_interface::{InvokeUiSession, Session},
};

const FIRST_CONNECTION_ID: i32 = 1_000_000;
const FRAME_INTERVAL: Duration = Duration::from_millis(100);
const MAX_BROWSER_UPLOAD_BYTES: usize = 8 * 1024 * 1024;
const MAX_BROWSER_DOWNLOAD_BYTES: u64 = 512 * 1024 * 1024;
const MAX_TRANSFER_CHUNK_BYTES: usize = 192 * 1024;

#[derive(Clone, Debug, PartialEq, Eq)]
enum RuntimeState {
    Pending,
    Active,
    Closed,
    Failed,
}

impl RuntimeState {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Active => "active",
            Self::Closed => "closed",
            Self::Failed => "failed",
        }
    }

    fn is_disconnected(&self) -> bool {
        matches!(self, Self::Closed | Self::Failed)
    }
}

#[derive(Clone, Debug)]
struct RuntimeRecord {
    connection_id: i32,
    remote_id: String,
    name: String,
    platform: String,
    state: RuntimeState,
    error: String,
    ended_at: Option<i64>,
    keyboard: bool,
    clipboard: bool,
    audio: bool,
    file: bool,
    restart: bool,
    recording: bool,
    block_input: bool,
    privacy_mode: bool,
    view_only: bool,
    show_remote_cursor: bool,
    terminal: bool,
    terminal_supported: bool,
    terminal_status: String,
    terminal_output: String,
    terminal_error: String,
    terminal_exit_code: Option<i32>,
    transfer_status: String,
    transfer_direction: String,
    transfer_name: String,
    transfer_error: String,
    transfer_bytes: u64,
    transfer_size: u64,
    transfer_speed: u64,
    transfer_job_id: Option<i32>,
    transfer_temp_path: Option<String>,
    remote_directory: String,
    remote_entries: Vec<Value>,
    remote_directory_status: String,
    remote_directory_error: String,
    voice_call: bool,
    display_count: usize,
    current_display: usize,
    direct: Option<bool>,
    secured: bool,
    last_message: String,
}

#[derive(Clone)]
struct StoredFrame {
    sequence: u64,
    width: usize,
    height: usize,
    captured_at: Instant,
    png: Vec<u8>,
}

/// Metadata for the latest decoded frame. The image bytes stay behind the
/// native `corevia://` protocol so normal bridge responses remain small.
pub(crate) struct FrameDescriptor {
    pub(crate) display: usize,
    pub(crate) sequence: u64,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

static NEXT_FRAME_SEQUENCE: AtomicI32 = AtomicI32::new(1);

fn diagnostic_event(message: impl AsRef<str>) {
    let path = std::env::temp_dir().join("corevia-desk-native-session.log");
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(file, "{}", message.as_ref());
    }
}

fn frames() -> &'static Mutex<HashMap<(i32, usize), StoredFrame>> {
    static FRAMES: std::sync::OnceLock<Mutex<HashMap<(i32, usize), StoredFrame>>> =
        std::sync::OnceLock::new();
    FRAMES.get_or_init(|| Mutex::new(HashMap::new()))
}

fn next_frame_sequence() -> u64 {
    // The sequence is only used to bust the WebView image cache. Wrapping is
    // harmless and keeps the counter bounded for a very long-running client.
    NEXT_FRAME_SEQUENCE.fetch_add(1, Ordering::Relaxed) as u64
}

fn packed_rgba(rgba: &scrap::ImageRgb) -> Option<Vec<u8>> {
    if rgba.w == 0 || rgba.h == 0 {
        return None;
    }

    let channels = match rgba.fmt() {
        ImageFormat::Raw => 3,
        ImageFormat::ARGB | ImageFormat::ABGR => 4,
    };
    let row_bytes = rgba.w.checked_mul(channels)?;
    let source_stride = rgba.raw.len().checked_div(rgba.h)?;
    if source_stride < row_bytes {
        return None;
    }

    let capacity = rgba.w.checked_mul(rgba.h)?.checked_mul(4)?;
    let mut output = Vec::with_capacity(capacity);
    for row in 0..rgba.h {
        let start = row.checked_mul(source_stride)?;
        let end = start.checked_add(row_bytes)?;
        let source = rgba.raw.get(start..end)?;
        if channels == 3 {
            for pixel in source.chunks_exact(3) {
                output.extend_from_slice(pixel);
                output.push(255);
            }
        } else {
            for pixel in source.chunks_exact(4) {
                // libyuv's ARGB buffer is BGRA in memory on the desktop
                // targets. ABGR is already RGBA in memory.
                if matches!(rgba.fmt(), ImageFormat::ARGB) {
                    output.extend_from_slice(&[pixel[2], pixel[1], pixel[0], pixel[3]]);
                } else {
                    output.extend_from_slice(pixel);
                }
            }
        }
    }
    Some(output)
}

fn store_frame(connection_id: i32, display: usize, rgba: &scrap::ImageRgb) {
    if let Ok(existing) = frames().lock() {
        if existing
            .get(&(connection_id, display))
            .map(|frame| frame.captured_at.elapsed() < FRAME_INTERVAL)
            .unwrap_or(false)
        {
            return;
        }
    }

    let Some(pixels) = packed_rgba(rgba) else {
        hbb_common::log::warn!(
            "Corevia native frame ignored because its decoded buffer is invalid: connection_id={}, display={}, width={}, height={}",
            connection_id,
            display,
            rgba.w,
            rgba.h
        );
        return;
    };
    let mut png = Vec::new();
    let encoded = repng::Options::fastest(rgba.w as u32, rgba.h as u32)
        .build(&mut png)
        .and_then(|mut encoder| {
            encoder.write(&pixels)?;
            encoder.finish()
        });
    if let Err(error) = encoded {
        hbb_common::log::warn!(
            "Corevia native frame encoding failed: connection_id={}, display={}, error={}",
            connection_id,
            display,
            error
        );
        return;
    }

    if let Ok(mut stored) = frames().lock() {
        stored.insert(
            (connection_id, display),
            StoredFrame {
                sequence: next_frame_sequence(),
                width: rgba.w,
                height: rgba.h,
                captured_at: Instant::now(),
                png,
            },
        );
    }
}

pub(crate) fn frame_descriptor(connection_id: i32) -> Option<FrameDescriptor> {
    let stored = frames().lock().ok()?;
    stored
        .iter()
        .filter(|((id, _), _)| *id == connection_id)
        .max_by_key(|(_, frame)| frame.sequence)
        .map(|((_, display), frame)| FrameDescriptor {
            display: *display,
            sequence: frame.sequence,
            width: frame.width,
            height: frame.height,
        })
}

pub(crate) fn frame_bytes(connection_id: i32, display: usize) -> Option<Vec<u8>> {
    frames()
        .lock()
        .ok()?
        .get(&(connection_id, display))
        .map(|frame| frame.png.clone())
}

fn clear_frames(connection_id: i32) {
    if let Ok(mut stored) = frames().lock() {
        stored.retain(|(id, _), _| *id != connection_id);
    }
}

impl RuntimeRecord {
    fn new(connection_id: i32, remote_id: String) -> Self {
        Self {
            connection_id,
            name: remote_id.clone(),
            remote_id,
            platform: "Unknown".to_owned(),
            state: RuntimeState::Pending,
            error: String::new(),
            ended_at: None,
            keyboard: true,
            clipboard: true,
            audio: true,
            file: true,
            restart: false,
            recording: false,
            block_input: false,
            privacy_mode: false,
            view_only: false,
            show_remote_cursor: false,
            terminal: false,
            terminal_supported: false,
            terminal_status: "unavailable".to_owned(),
            terminal_output: String::new(),
            terminal_error: String::new(),
            terminal_exit_code: None,
            transfer_status: "idle".to_owned(),
            transfer_direction: String::new(),
            transfer_name: String::new(),
            transfer_error: String::new(),
            transfer_bytes: 0,
            transfer_size: 0,
            transfer_speed: 0,
            transfer_job_id: None,
            transfer_temp_path: None,
            remote_directory: String::new(),
            remote_entries: Vec::new(),
            remote_directory_status: "idle".to_owned(),
            remote_directory_error: String::new(),
            voice_call: false,
            display_count: 0,
            current_display: 0,
            direct: None,
            secured: true,
            last_message: String::new(),
        }
    }
}

#[derive(Clone, Default)]
pub(crate) struct WebSessionHandler {
    record: Option<Arc<Mutex<RuntimeRecord>>>,
}

impl WebSessionHandler {
    fn with_record(record: Arc<Mutex<RuntimeRecord>>) -> Self {
        Self {
            record: Some(record),
        }
    }

    fn update_record(&self, update: impl FnOnce(&mut RuntimeRecord)) {
        if let Some(record) = &self.record {
            if let Ok(mut record) = record.lock() {
                update(&mut record);
            }
        }
    }
}

struct WebSessionEntry {
    session: Session<WebSessionHandler>,
    record: Arc<Mutex<RuntimeRecord>>,
}

static NEXT_CONNECTION_ID: AtomicI32 = AtomicI32::new(FIRST_CONNECTION_ID);

fn sessions() -> &'static Mutex<HashMap<i32, WebSessionEntry>> {
    static SESSIONS: std::sync::OnceLock<Mutex<HashMap<i32, WebSessionEntry>>> =
        std::sync::OnceLock::new();
    SESSIONS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn existing_connection(
    registry: &HashMap<i32, WebSessionEntry>,
    remote_id: &str,
) -> Option<i32> {
    registry.values().find_map(|entry| {
        let record = entry.record.lock().ok()?;
        (record.remote_id == remote_id && !record.state.is_disconnected())
            .then_some(record.connection_id)
    })
}

/// Start the real Rust connection loop for an outgoing WebView request.
///
/// `force_relay` is kept separate from the peer ID so an explicit `/r` suffix
/// can select the existing ID/Relay route without ever being stored as part
/// of the peer identity.
pub(crate) fn start_connection(remote_id: String, force_relay: bool) -> Option<i32> {
    if let Ok(registry) = sessions().lock() {
        if let Some(existing) = existing_connection(&registry, &remote_id) {
            diagnostic_event(format!(
                "reuse connection_id={} remote_id={} result=existing-pending-or-active",
                existing, remote_id
            ));
            return Some(existing);
        }
    }

    let connection_id = NEXT_CONNECTION_ID.fetch_add(1, Ordering::Relaxed);
    diagnostic_event(format!("start connection_id={} remote_id={}", connection_id, remote_id));
    let record = Arc::new(Mutex::new(RuntimeRecord::new(
        connection_id,
        remote_id.clone(),
    )));
    let handler = WebSessionHandler::with_record(record.clone());
    let session: Session<WebSessionHandler> = Session {
        password: String::new(),
        server_keyboard_enabled: Arc::new(std::sync::RwLock::new(true)),
        server_file_transfer_enabled: Arc::new(std::sync::RwLock::new(true)),
        server_clipboard_enabled: Arc::new(std::sync::RwLock::new(true)),
        reconnect_count: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        ui_handler: handler,
        ..Default::default()
    };
    session.lc.write().ok()?.initialize(
        remote_id.clone(),
        ConnType::DEFAULT_CONN,
        None,
        force_relay,
        None,
        None,
        None,
    );

    let runner = session.clone();

    // Register before starting the worker.  A rendezvous failure can happen
    // immediately; inserting afterwards could lose the terminal state.
    if let Ok(mut registry) = sessions().lock() {
        if let Some(existing) = existing_connection(&registry, &remote_id) {
            diagnostic_event(format!(
                "reuse connection_id={} remote_id={} result=existing-after-initialize",
                existing, remote_id
            ));
            return Some(existing);
        }
        registry.insert(
            connection_id,
            WebSessionEntry {
                session,
                record: record.clone(),
            },
        );
        hbb_common::log::debug!(
            "Corevia native session registered: connection_id={}, remote_id={}, registry_size={}",
            connection_id,
            remote_id,
            registry.len()
        );
        diagnostic_event(format!("registered connection_id={} registry_size={}", connection_id, registry.len()));
    } else {
        if let Ok(mut record) = record.lock() {
            record.state = RuntimeState::Failed;
            record.error = "connection-registry-lock-failed".to_owned();
        }
        return None;
    }

    let round = runner.connection_round_state.lock().ok()?.new_round();
    hbb_common::log::debug!(
        "Corevia native session worker starting: connection_id={}, remote_id={}",
        connection_id,
        remote_id
    );
    diagnostic_event(format!("worker-start connection_id={}", connection_id));
    if let Err(error) = std::thread::Builder::new()
        .name(format!("corevia-connect-{connection_id}"))
        .spawn(move || {
            crate::ui_session_interface::io_loop(runner, round);
            finish(connection_id);
        })
    {
        if let Ok(mut registry) = sessions().lock() {
            registry.remove(&connection_id);
        }
        if let Ok(mut record) = record.lock() {
            record.state = RuntimeState::Failed;
            record.error = format!("connection-worker-start-failed: {error}");
        }
        hbb_common::log::error!(
            "Corevia native session worker failed to start: connection_id={}, error={}",
            connection_id,
            error
        );
        return None;
    }

    Some(connection_id)
}

fn finish(connection_id: i32) {
    if let Ok(registry) = sessions().lock() {
        if let Some(entry) = registry.get(&connection_id) {
            entry.session.update_record(|record| {
                match record.state {
                    RuntimeState::Pending => {
                        record.state = RuntimeState::Failed;
                        if record.error.is_empty() {
                            record.error = "connection-closed-before-handshake".to_owned();
                        }
                    }
                    RuntimeState::Active => record.state = RuntimeState::Closed,
                    RuntimeState::Closed | RuntimeState::Failed => {}
                }
                if record.ended_at.is_none() {
                    record.ended_at = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .ok()
                        .map(|duration| duration.as_secs() as i64);
                }
            });
            if let Ok(record) = entry.record.lock() {
                hbb_common::log::debug!(
                    "Corevia native session worker finished: connection_id={}, state={}, error={}",
                    connection_id,
                    record.state.as_str(),
                    if record.error.is_empty() { "none" } else { record.error.as_str() }
                );
                diagnostic_event(format!(
                    "worker-finish connection_id={} state={} error={}",
                    connection_id,
                    record.state.as_str(),
                    if record.error.is_empty() { "none" } else { record.error.as_str() }
                ));
            }
            if let Ok(mut record) = entry.record.lock() {
                if let Some(path) = record.transfer_temp_path.take() {
                    let _ = fs::remove_file(path);
                }
            }
        } else {
            hbb_common::log::warn!(
                "Corevia native session worker finished without registry entry: connection_id={}",
                connection_id
            );
        }
    }
    clear_frames(connection_id);
}

/// Return the outgoing sessions in the same shape used by the bridge's
/// connection manager snapshot.
pub(crate) fn snapshot_clients() -> Vec<Value> {
    let Ok(registry) = sessions().lock() else {
        hbb_common::log::warn!("Corevia native session snapshot could not lock registry");
        return Vec::new();
    };

    hbb_common::log::debug!(
        "Corevia native session snapshot: registry_size={}",
        registry.len()
    );
    diagnostic_event(format!("snapshot registry_size={}", registry.len()));

    registry
        .values()
        .filter_map(|entry| {
            let record = entry.record.lock().ok()?.clone();
            Some(json!({
                "id": record.connection_id,
                "authorized": record.state == RuntimeState::Active,
                "disconnected": record.state.is_disconnected(),
                "is_file_transfer": false,
                "is_view_camera": false,
                "is_terminal": record.terminal,
                "terminal_supported": record.terminal_supported,
                "terminal_status": record.terminal_status,
                "port_forward": "",
                "name": record.name,
                "avatar": "",
                "peer_id": record.remote_id,
                "incoming": false,
                "keyboard": record.keyboard,
                "clipboard": record.clipboard,
                "audio": record.audio,
                "file": record.file,
                "restart": record.restart,
                "recording": record.recording,
                "block_input": record.block_input,
                "privacy_mode": record.privacy_mode,
                "from_switch": false,
                "in_voice_call": record.voice_call,
                "incoming_voice_call": false,
                "show_remote_cursor": record.show_remote_cursor,
                "view_only": record.view_only,
                "display_count": record.display_count,
                "current_display": record.current_display,
                "direct": record.direct,
                "secured": record.secured,
                "last_message": if record.last_message.is_empty() { Value::Null } else { json!(record.last_message) },
                "platform": record.platform,
                "state": record.state.as_str(),
                "error": if record.error.is_empty() { Value::Null } else { json!(record.error) },
                "endedAt": record.ended_at
            }))
        })
        .collect()
}

/// Ask a real outgoing session to close.
pub(crate) fn close(connection_id: i32) -> bool {
    let Ok(registry) = sessions().lock() else {
        diagnostic_event(format!("close connection_id={} result=registry-lock-failed", connection_id));
        return false;
    };
    let Some(entry) = registry.get(&connection_id) else {
        diagnostic_event(format!("close connection_id={} result=not-found", connection_id));
        return false;
    };
    diagnostic_event(format!("close connection_id={} result=send-close", connection_id));
    entry.session.send(Data::Close);
    true
}

fn active_entry(connection_id: i32) -> Option<Session<WebSessionHandler>> {
    let registry = sessions().lock().ok()?;
    let entry = registry.get(&connection_id)?;
    let record = entry.record.lock().ok()?;
    (record.state == RuntimeState::Active).then(|| entry.session.clone())
}

fn active_entry_with_record(
    connection_id: i32,
) -> Option<(Session<WebSessionHandler>, Arc<Mutex<RuntimeRecord>>)> {
    let registry = sessions().lock().ok()?;
    let entry = registry.get(&connection_id)?;
    let record = entry.record.lock().ok()?;
    (record.state == RuntimeState::Active)
        .then(|| (entry.session.clone(), entry.record.clone()))
}

fn bool_option(enabled: bool) -> BoolOption {
    if enabled {
        BoolOption::Yes
    } else {
        BoolOption::No
    }
}

fn send_option(session: &Session<WebSessionHandler>, option: OptionMessage) {
    let mut misc = Misc::new();
    misc.set_option(option);
    let mut message = Message::new();
    message.set_misc(misc);
    session.send(Data::Message(message));
}

/// Apply a permission/control toggle to an outgoing native session. The old
/// CM API only knows about incoming connections, so outgoing WebView sessions
/// must go through the Session's real Rust message path.
pub(crate) fn change_permission(connection_id: i32, name: &str, enabled: bool) -> bool {
    let Some((session, record)) = active_entry_with_record(connection_id) else {
        return false;
    };

    match name {
        "keyboard" => {
            let mut option = OptionMessage::new();
            option.disable_keyboard = bool_option(!enabled).into();
            send_option(&session, option);
            if let Ok(mut record) = record.lock() {
                record.view_only = !enabled && record.view_only;
            }
        }
        "clipboard" => {
            let mut option = OptionMessage::new();
            option.disable_clipboard = bool_option(!enabled).into();
            send_option(&session, option);
        }
        "audio" => {
            let mut option = OptionMessage::new();
            option.disable_audio = bool_option(!enabled).into();
            send_option(&session, option);
        }
        "file" => {
            let mut option = OptionMessage::new();
            option.enable_file_transfer = bool_option(enabled).into();
            send_option(&session, option);
        }
        "view_only" => {
            let mut option = OptionMessage::new();
            option.disable_keyboard = bool_option(enabled).into();
            option.disable_clipboard = bool_option(enabled).into();
            option.enable_file_transfer = bool_option(!enabled).into();
            option.show_remote_cursor = bool_option(enabled).into();
            send_option(&session, option);
            if let Ok(mut record) = record.lock() {
                record.view_only = enabled;
                record.keyboard = !enabled;
                record.clipboard = !enabled;
                record.file = !enabled;
                record.show_remote_cursor = enabled;
            }
        }
        "show_remote_cursor" => {
            let mut option = OptionMessage::new();
            option.show_remote_cursor = bool_option(enabled).into();
            send_option(&session, option);
            if let Ok(mut record) = record.lock() {
                record.show_remote_cursor = enabled;
            }
        }
        "block_input" => {
            let mut option = OptionMessage::new();
            option.block_input = bool_option(enabled).into();
            send_option(&session, option);
            if let Ok(mut record) = record.lock() {
                record.block_input = enabled;
            }
        }
        "recording" => {
            session.record_screen(enabled);
            if let Ok(mut record) = record.lock() {
                record.recording = enabled;
            }
        }
        "privacy_mode" => {
            session.toggle_privacy_mode("privacy_mode_impl_mag".to_owned(), enabled);
            if let Ok(mut record) = record.lock() {
                record.privacy_mode = enabled;
            }
        }
        "restart" => {
            session.restart_remote_device();
        }
        _ => return false,
    }
    true
}

pub(crate) fn send_chat(connection_id: i32, message: String) -> bool {
    let Some((session, record)) = active_entry_with_record(connection_id) else {
        return false;
    };
    session.send_chat(message.clone());
    if let Ok(mut record) = record.lock() {
        record.last_message = message;
    }
    true
}

pub(crate) fn record_screen(connection_id: i32, enabled: bool) -> bool {
    change_permission(connection_id, "recording", enabled)
}

pub(crate) fn toggle_privacy_mode(connection_id: i32, enabled: bool) -> bool {
    change_permission(connection_id, "privacy_mode", enabled)
}

pub(crate) fn restart_remote_device(connection_id: i32) -> bool {
    change_permission(connection_id, "restart", true)
}

pub(crate) fn switch_display(connection_id: i32, display: i32) -> bool {
    let Some((session, record)) = active_entry_with_record(connection_id) else {
        return false;
    };
    if display < 0 {
        return false;
    }
    session.switch_display(display);
    if let Ok(mut record) = record.lock() {
        record.current_display = display as usize;
    }
    true
}

pub(crate) fn refresh_video(connection_id: i32) -> bool {
    let Some((session, _)) = active_entry_with_record(connection_id) else {
        return false;
    };
    session.refresh_video(0);
    true
}

pub(crate) fn set_image_quality(connection_id: i32, quality: String) -> bool {
    let Some((session, _)) = active_entry_with_record(connection_id) else {
        return false;
    };
    if !matches!(quality.as_str(), "low" | "balanced" | "best" | "custom") {
        return false;
    }
    session.save_image_quality(quality);
    true
}

pub(crate) fn set_scroll_style(connection_id: i32, style: &str) -> bool {
    let Some((session, _)) = active_entry_with_record(connection_id) else {
        return false;
    };
    if !matches!(style, "auto" | "reverse" | "edge") {
        return false;
    }
    let updated = session
        .lc
        .write()
        .map(|mut lc| lc.save_scroll_style(style.to_owned()))
        .is_ok();
    updated
}

pub(crate) fn add_port_forward(
    connection_id: i32,
    local_port: i32,
    remote_host: String,
    remote_port: i32,
) -> bool {
    let Some((session, _)) = active_entry_with_record(connection_id) else {
        return false;
    };
    if !(1..=65535).contains(&local_port) || !(1..=65535).contains(&remote_port) {
        return false;
    }
    if remote_host.is_empty() || remote_host.len() > 255 {
        return false;
    }
    session.add_port_forward(local_port, remote_host, remote_port);
    true
}

pub(crate) fn elevate(connection_id: i32) -> bool {
    let Some((session, _)) = active_entry_with_record(connection_id) else {
        return false;
    };
    session.send(Data::ElevateDirect);
    true
}

pub(crate) fn open_terminal(connection_id: i32) -> bool {
    let Some((session, record)) = active_entry_with_record(connection_id) else {
        return false;
    };
    let Ok(mut record_state) = record.lock() else {
        return false;
    };
    if !record_state.terminal_supported {
        record_state.terminal_status = "unavailable".to_owned();
        record_state.terminal_error = "The remote device does not advertise terminal support.".to_owned();
        return false;
    }
    if record_state.terminal_status == "opening" || record_state.terminal_status == "open" {
        return true;
    }
    record_state.terminal = false;
    record_state.terminal_status = "opening".to_owned();
    record_state.terminal_output.clear();
    record_state.terminal_error.clear();
    record_state.terminal_exit_code = None;
    drop(record_state);
    session.open_terminal(1, 24, 80);
    true
}

pub(crate) fn close_terminal(connection_id: i32) -> bool {
    let Some((session, record)) = active_entry_with_record(connection_id) else {
        return false;
    };
    let Ok(mut record_state) = record.lock() else {
        return false;
    };
    if record_state.terminal_status != "open" && record_state.terminal_status != "opening" {
        return false;
    }
    record_state.terminal_status = "closing".to_owned();
    drop(record_state);
    session.close_terminal(1);
    true
}

pub(crate) fn send_terminal_input(connection_id: i32, input: &str) -> bool {
    if input.is_empty() || input.len() > 4096 || input.chars().any(|character| character == '\0') {
        return false;
    }
    let Some((session, record)) = active_entry_with_record(connection_id) else {
        return false;
    };
    let Ok(record_state) = record.lock() else {
        return false;
    };
    if record_state.terminal_status != "open" {
        return false;
    }
    drop(record_state);
    session.send_terminal_input(1, input.to_owned());
    true
}

pub(crate) fn terminal_snapshot(connection_id: i32) -> Option<Value> {
    let (_, record) = active_entry_with_record(connection_id)?;
    let record = record.lock().ok()?;
    Some(json!({
        "open": record.terminal,
        "supported": record.terminal_supported,
        "status": record.terminal_status,
        "output": record.terminal_output,
        "error": record.terminal_error,
        "exitCode": record.terminal_exit_code
    }))
}

fn safe_file_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 255
        && name != "."
        && name != ".."
        && !name.chars().any(|character| {
            character.is_control() || matches!(character, '/' | '\\' | ':' | '\0')
        })
}

fn safe_remote_directory(path: &str, windows: bool) -> bool {
    if path.is_empty() || path.len() > 4096 || path.chars().any(char::is_control) {
        return false;
    }
    let normalized = path.replace('\\', "/");
    if normalized.split('/').any(|part| part == "..") {
        return false;
    }
    if windows {
        let bytes = normalized.as_bytes();
        (bytes.len() >= 3
            && bytes[0].is_ascii_alphabetic()
            && bytes[1] == b':'
            && bytes[2] == b'/')
            || normalized.starts_with("//")
    } else {
        normalized.starts_with('/')
    }
}

fn remote_child_path(directory: &str, name: &str, separator: &str) -> String {
    format!("{}{}{}", directory.trim_end_matches(['/', '\\']), separator, name)
}

fn update_transfer_result(
    record: &Arc<Mutex<RuntimeRecord>>,
    status: &str,
    error: Option<&str>,
    remove_temp: bool,
) {
    if let Ok(mut record) = record.lock() {
        record.transfer_status = status.to_owned();
        record.transfer_error = error.unwrap_or_default().to_owned();
        record.transfer_job_id = None;
        if remove_temp {
            if let Some(path) = record.transfer_temp_path.take() {
                let _ = fs::remove_file(path);
            }
        }
    }
}

/// Request a listing for a remote directory; the native file callback stores
/// the result for the Nuxt-only snapshot operation.
#[cfg(not(any(target_os = "android", target_os = "ios", feature = "flutter")))]
pub(crate) fn read_remote_folder(connection_id: i32, path: &str) -> bool {
    let Some((session, record)) = active_entry_with_record(connection_id) else {
        return false;
    };
    let platform = session.peer_platform().to_ascii_lowercase();
    if !safe_remote_directory(path, platform.contains("windows")) {
        return false;
    }
    if let Ok(mut state) = record.lock() {
        if !state.file {
            return false;
        }
        state.remote_directory = path.to_owned();
        state.remote_entries.clear();
        state.remote_directory_status = "loading".to_owned();
        state.remote_directory_error.clear();
    }
    session.read_remote_dir(path.to_owned(), false);
    true
}

#[cfg(any(target_os = "android", target_os = "ios", feature = "flutter"))]
pub(crate) fn read_remote_folder(_connection_id: i32, _path: &str) -> bool {
    false
}

/// Stage one bounded browser-selected file in a unique temporary file, then
/// pass it to the existing RustDesk transfer job without exposing a local path
/// or a generic filesystem operation to the bridge.
#[cfg(not(any(target_os = "android", target_os = "ios", feature = "flutter")))]
pub(crate) fn upload_file(
    connection_id: i32,
    name: &str,
    remote_directory: &str,
    encoded_content: &str,
) -> bool {
    if !safe_file_name(name)
        || encoded_content.len() > MAX_BROWSER_UPLOAD_BYTES.div_ceil(3) * 4 + 8
    {
        return false;
    }
    let Ok(content) = crate::decode64(encoded_content) else {
        return false;
    };
    if content.is_empty() || content.len() > MAX_BROWSER_UPLOAD_BYTES {
        return false;
    }
    let Some((session, record)) = active_entry_with_record(connection_id) else {
        return false;
    };
    let platform = session.peer_platform().to_ascii_lowercase();
    if !safe_remote_directory(remote_directory, platform.contains("windows")) {
        return false;
    }
    let (job_id, temp_path, destination) = {
        let Ok(mut state) = record.lock() else {
            return false;
        };
        if !state.file || state.transfer_job_id.is_some() || state.transfer_temp_path.is_some() {
            return false;
        }
        let temp_path = std::env::temp_dir().join(format!(
            "corevia-desk-transfer-{}.tmp",
            uuid::Uuid::new_v4()
        ));
        let Ok(mut file) = OpenOptions::new().write(true).create_new(true).open(&temp_path)
        else {
            return false;
        };
        if file.write_all(&content).is_err() || file.flush().is_err() {
            let _ = fs::remove_file(&temp_path);
            return false;
        }
        let Ok(temp_metadata) = file.metadata() else {
            let _ = fs::remove_file(&temp_path);
            return false;
        };
        if temp_metadata.len() as usize != content.len() {
            let _ = fs::remove_file(&temp_path);
            return false;
        }
        let job_id = hbb_common::fs::get_next_job_id();
        hbb_common::fs::update_next_job_id(job_id + 1);
        state.transfer_status = "sending".to_owned();
        state.transfer_direction = "upload".to_owned();
        state.transfer_name = name.to_owned();
        state.transfer_error.clear();
        state.transfer_bytes = 0;
        state.transfer_size = content.len() as u64;
        state.transfer_speed = 0;
        state.transfer_job_id = Some(job_id);
        state.transfer_temp_path = Some(temp_path.to_string_lossy().into_owned());
        let destination = remote_child_path(remote_directory, name, session.get_path_sep(true));
        (job_id, temp_path, destination)
    };
    session.send_files(
        job_id,
        0,
        temp_path.to_string_lossy().into_owned(),
        destination,
        0,
        false,
        false,
    );
    true
}

#[cfg(any(target_os = "android", target_os = "ios", feature = "flutter"))]
pub(crate) fn upload_file(
    _connection_id: i32,
    _name: &str,
    _remote_directory: &str,
    _encoded_content: &str,
) -> bool {
    false
}

/// Download one regular file returned by the latest remote directory listing.
/// A listed path is required, symbolic links and oversized files are rejected,
/// and bytes remain in a unique temporary file until the UI reads the chunks.
#[cfg(not(any(target_os = "android", target_os = "ios", feature = "flutter")))]
pub(crate) fn download_remote_file(connection_id: i32, name: &str) -> bool {
    if !safe_file_name(name) {
        return false;
    }
    let Some((session, record)) = active_entry_with_record(connection_id) else {
        return false;
    };
    let (job_id, temp_path, remote_path, size) = {
        let Ok(mut state) = record.lock() else {
            return false;
        };
        if !state.file || state.transfer_job_id.is_some() || state.transfer_temp_path.is_some() {
            return false;
        }
        let Some(entry) = state.remote_entries.iter().find(|entry| {
            entry.get("name").and_then(Value::as_str) == Some(name)
                && entry.get("entryType").and_then(Value::as_i64) == Some(4)
        }) else {
            return false;
        };
        let size = entry.get("size").and_then(Value::as_u64).unwrap_or(0);
        if size == 0 || size > MAX_BROWSER_DOWNLOAD_BYTES {
            return false;
        }
        let platform = session.peer_platform().to_ascii_lowercase();
        if !safe_remote_directory(&state.remote_directory, platform.contains("windows")) {
            return false;
        }
        let remote_path = remote_child_path(
            &state.remote_directory,
            name,
            session.get_path_sep(true),
        );
        let temp_path = std::env::temp_dir().join(format!(
            "corevia-desk-transfer-{}.tmp",
            uuid::Uuid::new_v4()
        ));
        let Ok(_file) = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp_path)
        else {
            return false;
        };
        let job_id = hbb_common::fs::get_next_job_id();
        hbb_common::fs::update_next_job_id(job_id + 1);
        state.transfer_status = "receiving".to_owned();
        state.transfer_direction = "download".to_owned();
        state.transfer_name = name.to_owned();
        state.transfer_error.clear();
        state.transfer_bytes = 0;
        state.transfer_size = size;
        state.transfer_speed = 0;
        state.transfer_job_id = Some(job_id);
        state.transfer_temp_path = Some(temp_path.to_string_lossy().into_owned());
        (job_id, temp_path, remote_path, size)
    };
    let _ = size;
    session.send_files(
        job_id,
        0,
        remote_path,
        temp_path.to_string_lossy().into_owned(),
        0,
        false,
        true,
    );
    true
}

#[cfg(any(target_os = "android", target_os = "ios", feature = "flutter"))]
pub(crate) fn download_remote_file(_connection_id: i32, _name: &str) -> bool {
    false
}

pub(crate) fn transfer_snapshot(connection_id: i32) -> Option<Value> {
    let registry = sessions().lock().ok()?;
    let entry = registry.get(&connection_id)?;
    let record = entry.record.lock().ok()?;
    Some(json!({
        "status": record.transfer_status,
        "direction": record.transfer_direction,
        "name": record.transfer_name,
        "error": record.transfer_error,
        "bytes": record.transfer_bytes,
        "size": record.transfer_size,
        "speed": record.transfer_speed,
        "remoteDirectory": record.remote_directory,
        "remoteDirectoryStatus": record.remote_directory_status,
        "remoteDirectoryError": record.remote_directory_error,
        "entries": record.remote_entries
    }))
}

/// Read one bounded piece of a completed transfer into a browser download.
pub(crate) fn read_transfer_chunk(
    connection_id: i32,
    offset: u64,
    requested_bytes: usize,
) -> Option<Value> {
    if requested_bytes == 0 || requested_bytes > MAX_TRANSFER_CHUNK_BYTES {
        return None;
    }
    let path = {
        let registry = sessions().lock().ok()?;
        let entry = registry.get(&connection_id)?;
        let record = entry.record.lock().ok()?;
        if record.transfer_status != "done" || record.transfer_direction != "download" {
            return None;
        }
        if offset > record.transfer_size || record.transfer_size > MAX_BROWSER_DOWNLOAD_BYTES {
            return None;
        }
        record.transfer_temp_path.clone()?
    };
    let mut file = File::open(path).ok()?;
    let total_size = file.metadata().ok()?.len();
    file.seek(SeekFrom::Start(offset)).ok()?;
    let mut bytes = vec![0; requested_bytes.min(MAX_TRANSFER_CHUNK_BYTES)];
    let read = file.read(&mut bytes).ok()?;
    bytes.truncate(read);
    Some(json!({
        "offset": offset,
        "dataBase64": crate::encode64(bytes),
        "eof": read == 0 || offset.saturating_add(read as u64) >= total_size
    }))
}

pub(crate) fn cleanup_transfer(connection_id: i32) -> bool {
    let Ok(registry) = sessions().lock() else {
        return false;
    };
    let Some(entry) = registry.get(&connection_id) else {
        return false;
    };
    let Ok(mut record) = entry.record.lock() else {
        return false;
    };
    if record.transfer_job_id.is_some() {
        return false;
    }
    if let Some(path) = record.transfer_temp_path.take() {
        let _ = fs::remove_file(path);
    }
    record.transfer_status = "idle".to_owned();
    record.transfer_name.clear();
    record.transfer_error.clear();
    record.transfer_bytes = 0;
    record.transfer_size = 0;
    true
}

pub(crate) fn request_voice_call(connection_id: i32) -> bool {
    let Some((session, record)) = active_entry_with_record(connection_id) else {
        return false;
    };
    session.request_voice_call();
    if let Ok(mut record) = record.lock() {
        record.voice_call = true;
    }
    true
}

pub(crate) fn close_voice_call(connection_id: i32) -> bool {
    let Some((session, record)) = active_entry_with_record(connection_id) else {
        return false;
    };
    session.close_voice_call();
    if let Ok(mut record) = record.lock() {
        record.voice_call = false;
    }
    true
}

pub(crate) fn send_mouse(
    connection_id: i32,
    mask: i32,
    x: i32,
    y: i32,
    alt: bool,
    ctrl: bool,
    shift: bool,
    command: bool,
) -> bool {
    let Some(session) = active_entry(connection_id) else {
        return false;
    };
    session.send_mouse(mask, x, y, alt, ctrl, shift, command);
    true
}

pub(crate) fn send_key(
    connection_id: i32,
    name: &str,
    down: bool,
    press: bool,
    alt: bool,
    ctrl: bool,
    shift: bool,
    command: bool,
) -> bool {
    let Some(session) = active_entry(connection_id) else {
        return false;
    };
    session.input_key(name, down, press, alt, ctrl, shift, command);
    true
}

impl InvokeUiSession for WebSessionHandler {
    fn set_cursor_data(&self, _cd: CursorData) {}

    fn set_cursor_id(&self, _id: String) {}

    fn set_cursor_position(&self, _cp: CursorPosition) {}

    fn set_display(
        &self,
        _x: i32,
        _y: i32,
        _w: i32,
        _h: i32,
        _cursor_embedded: bool,
        _scale: f64,
    ) {
    }

    fn switch_display(&self, _display: &SwitchDisplay) {}

    fn set_peer_info(&self, peer_info: &PeerInfo) {
        diagnostic_event(format!(
            "peer-info remote_id={} username={} platform={}",
            self.record
                .as_ref()
                .and_then(|record| record.lock().ok().map(|record| record.remote_id.clone()))
                .unwrap_or_default(),
            peer_info.username,
            peer_info.platform
        ));
        self.update_record(|record| {
            record.name = if peer_info.username.is_empty() {
                record.remote_id.clone()
            } else {
                peer_info.username.clone()
            };
            record.platform = if peer_info.platform.is_empty() {
                "Unknown".to_owned()
            } else {
                peer_info.platform.clone()
            };
            record.terminal_supported = peer_info
                .features
                .as_ref()
                .is_some_and(|features| features.terminal);
            if !record.terminal_supported && record.terminal_status == "unavailable" {
                record.terminal_error.clear();
            }
        });
    }

    fn set_displays(&self, displays: &Vec<DisplayInfo>) {
        diagnostic_event(format!(
            "displays count={} remote_id={}",
            displays.len(),
            self.record
                .as_ref()
                .and_then(|record| record.lock().ok().map(|record| record.remote_id.clone()))
                .unwrap_or_default()
        ));
        self.update_record(|record| {
            record.display_count = displays.len();
        });
    }

    fn set_platform_additions(&self, _data: &str) {}

    fn on_connected(&self, _conn_type: ConnType) {
        if let Some(peer_id) = self
            .record
            .as_ref()
            .and_then(|record| record.lock().ok().map(|record| record.remote_id.clone()))
        {
            crate::corevia_bridge::mark_recent_peer_connection(&peer_id);
        }
        diagnostic_event(format!(
            "connected remote_id={}",
            self.record
                .as_ref()
                .and_then(|record| record.lock().ok().map(|record| record.remote_id.clone()))
                .unwrap_or_default()
        ));
        self.update_record(|record| {
            record.state = RuntimeState::Active;
            record.error.clear();
        });
    }

    fn update_privacy_mode(&self) {}

    fn set_permission(&self, name: &str, value: bool) {
        self.update_record(|record| match name {
            "keyboard" => record.keyboard = value,
            "clipboard" => record.clipboard = value,
            "audio" => record.audio = value,
            "file" => record.file = value,
            "restart" => record.restart = value,
            "recording" => record.recording = value,
            "block_input" => record.block_input = value,
            "privacy_mode" => record.privacy_mode = value,
            "show_remote_cursor" => record.show_remote_cursor = value,
            _ => {}
        });
    }

    fn close_success(&self) {
        let connection_id = self
            .record
            .as_ref()
            .and_then(|record| record.lock().ok().map(|record| record.connection_id));
        diagnostic_event(format!(
            "first-frame-ready connection_id={} ",
            connection_id.map(|id| id.to_string()).unwrap_or_else(|| "unknown".to_owned())
        ));
        // RustDesk calls this callback when the first video frame is ready.
        // The native WebView runtime must keep the transport alive here; the
        // explicit close operation and the worker's finalizer own teardown.
        self.update_record(|record| {
            if matches!(record.state, RuntimeState::Pending) {
                record.state = RuntimeState::Active;
            }
            record.last_message = "Connected".to_owned();
        });
    }

    fn update_quality_status(&self, _qs: QualityStatus) {}

    fn set_connection_type(&self, is_secured: bool, direct: bool, _stream_type: &str) {
        self.update_record(|record| {
            record.secured = is_secured;
            record.direct = Some(direct);
        });
    }

    fn set_fingerprint(&self, _fingerprint: String) {}

    fn job_error(&self, id: i32, err: String, _file_num: i32) {
        let Some(record) = &self.record else {
            return;
        };
        let path = if let Ok(mut state) = record.lock() {
            if state.transfer_job_id != Some(id) {
                return;
            }
            state.transfer_status = "error".to_owned();
            state.transfer_error = err;
            state.transfer_job_id = None;
            state.transfer_temp_path.take()
        } else {
            None
        };
        if let Some(path) = path {
            let _ = fs::remove_file(path);
        }
    }

    fn job_done(&self, id: i32, _file_num: i32) {
        let Some(record) = &self.record else {
            return;
        };
        let path = if let Ok(mut state) = record.lock() {
            if state.transfer_job_id != Some(id) {
                return;
            }
            state.transfer_status = "done".to_owned();
            state.transfer_error.clear();
            state.transfer_bytes = state.transfer_size;
            state.transfer_job_id = None;
            if state.transfer_direction == "upload" {
                state.transfer_temp_path.take()
            } else {
                None
            }
        } else {
            None
        };
        if let Some(path) = path {
            let _ = fs::remove_file(path);
        }
    }

    fn clear_all_jobs(&self) {}

    fn new_message(&self, msg: String) {
        self.update_record(|record| record.last_message = msg);
    }

    fn update_transfer_list(&self) {}

    fn load_last_job(&self, _cnt: i32, _job_json: &str, _auto_start: bool) {}

    fn update_folder_files(
        &self,
        _id: i32,
        entries: &Vec<FileEntry>,
        path: String,
        is_local: bool,
        only_count: bool,
    ) {
        if is_local || only_count {
            return;
        }
        self.update_record(|record| {
            if record.remote_directory != path {
                return;
            }
            record.remote_entries = entries
                .iter()
                .map(|entry| {
                    json!({
                        "name": entry.name,
                        "entryType": entry.entry_type.value(),
                        "size": entry.size,
                        "modifiedTime": entry.modified_time,
                        "hidden": entry.is_hidden
                    })
                })
                .collect();
            record.remote_directory_status = "loaded".to_owned();
            record.remote_directory_error.clear();
        });
    }

    fn confirm_delete_files(&self, _id: i32, _i: i32, _name: String) {}

    fn override_file_confirm(
        &self,
        _id: i32,
        _file_num: i32,
        _to: String,
        _is_upload: bool,
        _is_identical: bool,
    ) {
    }

    fn update_block_input_state(&self, on: bool) {
        self.update_record(|record| record.block_input = on);
    }

    fn job_progress(&self, id: i32, _file_num: i32, speed: f64, finished_size: f64) {
        self.update_record(|record| {
            if record.transfer_job_id == Some(id) {
                record.transfer_bytes = finished_size.max(0.0) as u64;
                record.transfer_speed = speed.max(0.0) as u64;
            }
        });
    }

    fn adapt_size(&self) {}

    fn on_rgba(&self, display: usize, rgba: &mut scrap::ImageRgb) {
        let connection_id = self
            .record
            .as_ref()
            .and_then(|record| record.lock().ok().map(|record| record.connection_id));
        if let Some(connection_id) = connection_id {
            diagnostic_event(format!(
                "rgba connection_id={} display={} width={} height={}",
                connection_id, display, rgba.w, rgba.h
            ));
            store_frame(connection_id, display, rgba);
        }
    }

    fn msgbox(&self, msgtype: &str, _title: &str, text: &str, _link: &str, _retry: bool) {
        let connection_id = self
            .record
            .as_ref()
            .and_then(|record| record.lock().ok().map(|record| record.connection_id));
        diagnostic_event(format!(
            "msgbox connection_id={} type={} text={}",
            connection_id.map(|id| id.to_string()).unwrap_or_else(|| "unknown".to_owned()),
            msgtype,
            text.replace('\n', " ")
        ));
        if msgtype == "error" {
            self.update_record(|record| {
                record.state = RuntimeState::Failed;
                record.error = text.to_owned();
            });
        }
    }

    fn cancel_msgbox(&self, _tag: &str) {}

    fn switch_back(&self, _id: &str) {}

    fn portable_service_running(&self, _running: bool) {}

    fn on_voice_call_started(&self) {
        self.update_record(|record| record.voice_call = true);
    }

    fn on_voice_call_closed(&self, _reason: &str) {
        self.update_record(|record| record.voice_call = false);
    }

    fn on_voice_call_waiting(&self) {}

    fn on_voice_call_incoming(&self) {}

    fn get_rgba(&self, _display: usize) -> *const u8 {
        std::ptr::null()
    }

    fn next_rgba(&self, _display: usize) {}

    fn set_multiple_windows_session(&self, _sessions: Vec<WindowsSession>) {}

    fn set_current_display(&self, disp_idx: i32) {
        if disp_idx >= 0 {
            self.update_record(|record| record.current_display = disp_idx as usize);
        }
    }

    fn update_record_status(&self, start: bool) {
        self.update_record(|record| record.recording = start);
    }

    fn printer_request(&self, _id: i32, _path: String) {}

    fn handle_screenshot_resp(&self, _sid: String, _msg: String) {}

    fn handle_terminal_response(&self, response: TerminalResponse) {
        use hbb_common::message_proto::terminal_response::Union;

        match response.union {
            Some(Union::Opened(opened)) => self.update_record(|record| {
                record.terminal = opened.success;
                record.terminal_status = if opened.success { "open" } else { "error" }.to_owned();
                record.terminal_error = if opened.success {
                    String::new()
                } else {
                    opened.message
                };
            }),
            Some(Union::Data(data)) => {
                let output = if data.compressed {
                    hbb_common::compress::decompress(&data.data)
                } else {
                    data.data.to_vec()
                };
                let output = String::from_utf8_lossy(&output).into_owned();
                self.update_record(|record| {
                    if data.terminal_id != 1 || record.terminal_status != "open" {
                        return;
                    }
                    record.terminal_output.push_str(&output);
                    const MAX_TERMINAL_OUTPUT_BYTES: usize = 64 * 1024;
                    if record.terminal_output.len() > MAX_TERMINAL_OUTPUT_BYTES {
                        let mut trim_to = record.terminal_output.len() - MAX_TERMINAL_OUTPUT_BYTES;
                        while !record.terminal_output.is_char_boundary(trim_to) {
                            trim_to += 1;
                        }
                        record.terminal_output.drain(..trim_to);
                    }
                });
            }
            Some(Union::Closed(closed)) => self.update_record(|record| {
                if closed.terminal_id == 1 {
                    record.terminal = false;
                    record.terminal_status = "closed".to_owned();
                    record.terminal_exit_code = Some(closed.exit_code);
                }
            }),
            Some(Union::Error(error)) => self.update_record(|record| {
                if error.terminal_id == 1 {
                    record.terminal = false;
                    record.terminal_status = "error".to_owned();
                    record.terminal_error = error.message;
                }
            }),
            None => {}
            Some(_) => {}
        }
    }
}

// Keep the `Client` import tied to this module's compile-time contract. The
// connection loop uses it through `Session` and the import also makes it
// explicit that this adapter is Rust-native rather than a UI shim.
#[allow(dead_code)]
fn _rust_client_type_marker(_: &Client) {}

#[cfg(test)]
mod terminal_response_tests {
    use super::*;
    use hbb_common::message_proto::{
        terminal_response::Union, TerminalClosed, TerminalData, TerminalOpened,
    };

    #[test]
    fn terminal_response_updates_open_state_output_and_exit_status() {
        let record = Arc::new(Mutex::new(RuntimeRecord::new(7, "test-peer".to_owned())));
        let handler = WebSessionHandler::with_record(record.clone());

        handler.handle_terminal_response(TerminalResponse {
            union: Some(Union::Opened(TerminalOpened {
                terminal_id: 1,
                success: true,
                ..Default::default()
            })),
            ..Default::default()
        });
        let opened = record.lock().unwrap().clone();
        assert!(opened.terminal);
        assert_eq!(opened.terminal_status, "open");

        handler.handle_terminal_response(TerminalResponse {
            union: Some(Union::Data(TerminalData {
                terminal_id: 1,
                data: bytes::Bytes::from_static(b"safe output\n"),
                compressed: false,
                ..Default::default()
            })),
            ..Default::default()
        });
        assert_eq!(record.lock().unwrap().terminal_output, "safe output\n");

        handler.handle_terminal_response(TerminalResponse {
            union: Some(Union::Closed(TerminalClosed {
                terminal_id: 1,
                exit_code: 0,
                ..Default::default()
            })),
            ..Default::default()
        });
        let closed = record.lock().unwrap().clone();
        assert!(!closed.terminal);
        assert_eq!(closed.terminal_status, "closed");
        assert_eq!(closed.terminal_exit_code, Some(0));
    }
}
