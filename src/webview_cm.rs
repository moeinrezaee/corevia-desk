//! Native WebView connection-manager host.
//!
//! RustDesk's server process hands incoming sessions to a user-side
//! connection manager over the `_cm` IPC endpoint.  The legacy Flutter and
//! Sciter hosts used to create that listener; the native Nuxt/WebView path
//! must own it explicitly as well.  This module keeps the transport and
//! approval state in the existing Rust connection-manager implementation and
//! leaves the visual surface to Nuxt.

use crate::ui_cm_interface::{ConnectionManager, InvokeUiCM};
use hbb_common::log;
use std::sync::Once;

#[derive(Clone, Default)]
struct WebViewCmHandler;

impl InvokeUiCM for WebViewCmHandler {
    fn add_connection(&self, client: &crate::ui_cm_interface::Client) {
        log::info!(
            "Corevia WebView connection manager received incoming connection: id={}, peer_id={}, authorized={}",
            client.id,
            client.peer_id,
            client.authorized
        );
    }

    fn remove_connection(&self, id: i32, close: bool) {
        log::info!(
            "Corevia WebView connection manager removed connection: id={}, close={}",
            id,
            close
        );
    }

    fn new_message(&self, id: i32, _text: String) {
        log::debug!(
            "Corevia WebView connection manager received chat message: id={}",
            id
        );
    }

    fn change_theme(&self, _dark: String) {}

    fn change_language(&self) {}

    fn show_elevation(&self, _show: bool) {}

    fn update_voice_call_state(&self, _client: &crate::ui_cm_interface::Client) {}

    fn file_transfer_log(&self, _action: &str, _log: &str) {}
}

static STARTED: Once = Once::new();

/// Start the user-side `_cm` listener once for the active native WebView
/// process.  The same entrypoint is used by the normal UI process and by the
/// user process launched by the RustDesk service when an incoming connection
/// arrives.
pub(crate) fn start() {
    STARTED.call_once(|| {
        std::thread::Builder::new()
            .name("corevia-webview-cm".to_owned())
            .spawn(|| {
                log::debug!("Corevia WebView connection manager listener starting");
                let cm = ConnectionManager {
                    ui_handler: WebViewCmHandler,
                };
                crate::ui_cm_interface::start_ipc(cm);
            })
            .expect("Corevia WebView connection manager could not start");
    });
}
