//! Native desktop host for the Corevia Desk Nuxt view.
//!
//! This is the active desktop entrypoint. The legacy Sciter and Flutter
//! sources remain in the repository for history and rollback, but this host
//! never starts either of them.

use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::{Icon, WindowBuilder},
};
use wry::{
    http::{header::CONTENT_TYPE, Request, Response},
    WebViewBuilder,
};

const PROTOCOL: &str = "corevia";
const START_URL: &str = "corevia://localhost/";
const WEB_ROOT_ENV: &str = "COREVIA_DESK_WEB_ROOT";

const BRIDGE_INITIALIZATION_SCRIPT: &str = r#"
(() => {
  const pending = new Map();
  const resolve = (response) => {
    const complete = pending.get(response?.requestId);
    if (!complete) return;
    pending.delete(response.requestId);
    complete(response);
  };

  Object.defineProperty(window, '__COREVIA_DESK_BRIDGE_INTERNAL__', {
    configurable: false,
    enumerable: false,
    value: { resolve }
  });

  window.__COREVIA_DESK_BRIDGE__ = {
    request(request) {
      return new Promise((complete, reject) => {
        if (!request || typeof request.requestId !== 'string') {
          reject(new Error('Invalid Corevia Desk bridge request.'));
          return;
        }
        if (!window.ipc || typeof window.ipc.postMessage !== 'function') {
          reject(new Error('The native Corevia Desk bridge is unavailable.'));
          return;
        }
        pending.set(request.requestId, complete);
        window.ipc.postMessage(JSON.stringify(request));
      });
    }
  };
})();
"#;

#[derive(Debug, Clone)]
enum NativeEvent {
    BridgeResponse(String),
    Activate,
}

/// Start the active Corevia Desk desktop shell.
pub fn start(args: &mut [String], instance_guard: Option<crate::single_instance::InstanceGuard>) {
    #[cfg(target_os = "macos")]
    crate::platform::delegate::show_dock();

    let web_root = match resolve_web_root() {
        Ok(path) => path,
        Err(error) => {
            hbb_common::log::error!("Corevia Desk Nuxt view could not be loaded: {}", error);
            eprintln!("Corevia Desk Nuxt view could not be loaded: {error}");
            return;
        }
    };

    let startup_args = serde_json::to_string(&args.to_vec()).unwrap_or_else(|_| "[]".to_owned());
    let initialization_script = format!(
        "{}\nwindow.__COREVIA_DESK_STARTUP_ARGS__ = {};",
        BRIDGE_INITIALIZATION_SCRIPT, startup_args
    );

    hbb_common::log::info!(
        "Starting Corevia Desk native WebView host with Nuxt assets from {}",
        web_root.display()
    );

    let event_loop = EventLoopBuilder::<NativeEvent>::with_user_event().build();
    let event_loop_proxy = event_loop.create_proxy();
    let window_icon = image::load_from_memory(include_bytes!("../res/128x128@2x.png"))
        .ok()
        .and_then(|image| {
            let image = image.into_rgba8();
            let (width, height) = image.dimensions();
            Icon::from_rgba(image.into_raw(), width, height).ok()
        });
    let compact_support = cfg!(feature = "corevia-customer-support");
    let full_desk_size = tao::dpi::LogicalSize::new(1440.0, 900.0);
    let support_size = event_loop
        .primary_monitor()
        .map(|monitor| {
            let screen = monitor.size().to_logical::<f64>(monitor.scale_factor());
            tao::dpi::LogicalSize::new(
                430.0_f64.min((screen.width - 32.0).max(360.0)),
                820.0_f64.min((screen.height - 64.0).max(500.0)),
            )
        })
        .unwrap_or_else(|| tao::dpi::LogicalSize::new(430.0, 820.0));
    let window = WindowBuilder::new()
        .with_title(crate::branding::APP_DISPLAY_NAME.to_owned())
        .with_window_icon(window_icon)
        .with_inner_size(if compact_support {
            support_size
        } else {
            full_desk_size
        })
        .with_min_inner_size(if compact_support {
            support_size
        } else {
            tao::dpi::LogicalSize::new(960.0, 640.0)
        })
        .with_resizable(!compact_support)
        .build(&event_loop)
        .expect("Corevia Desk native window could not be created");

    let mut instance_guard = instance_guard;
    if let Some(instance_guard) = instance_guard.as_mut() {
        if let Err(error) =
            instance_guard.start_listener(event_loop_proxy.clone(), NativeEvent::Activate)
        {
            hbb_common::log::error!("Could not start Corevia Desk activation listener: {error}");
            eprintln!("Could not start Corevia Desk activation listener: {error}");
            return;
        }
    }

    let protocol_root = web_root.clone();
    let ipc_proxy = event_loop_proxy.clone();
    let builder = WebViewBuilder::new()
        .with_custom_protocol(PROTOCOL.to_owned(), move |_webview_id, request| {
            serve_asset(&protocol_root, request)
        })
        .with_initialization_script(&initialization_script)
        .with_ipc_handler(move |request| {
            let response = dispatch_bridge_message(request.body());
            if ipc_proxy
                .send_event(NativeEvent::BridgeResponse(response))
                .is_err()
            {
                hbb_common::log::warn!("Corevia Desk bridge response could not reach the WebView");
            }
        })
        .with_on_page_load_handler(|event, url| {
            let stage = match event {
                wry::PageLoadEvent::Started => "started",
                wry::PageLoadEvent::Finished => "finished",
            };
            hbb_common::log::debug!("Corevia Desk Nuxt page load: {} {}", stage, url);
        })
        .with_url(START_URL);

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    let webview = builder
        .build(&window)
        .expect("Corevia Desk native WebView could not be created");

    #[cfg(target_os = "linux")]
    let webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;

        let vbox = window
            .default_vbox()
            .expect("Corevia Desk Linux window container could not be created");
        builder
            .build_gtk(vbox)
            .expect("Corevia Desk native WebView could not be created")
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::UserEvent(NativeEvent::Activate) => {
                if window.is_minimized() {
                    window.set_minimized(false);
                }
                window.set_visible(true);
                window.set_focus();
            }
            Event::UserEvent(NativeEvent::BridgeResponse(response)) => {
                let script = format!(
                    "window.__COREVIA_DESK_BRIDGE_INTERNAL__?.resolve({});",
                    response
                );
                if let Err(error) = webview.evaluate_script(&script) {
                    hbb_common::log::warn!("Corevia Desk bridge response script failed: {}", error);
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}

fn dispatch_bridge_message(message: &str) -> String {
    match serde_json::from_str::<crate::corevia_bridge::BridgeRequest>(message) {
        Ok(request) => serde_json::to_string(&crate::corevia_bridge::dispatch(request))
            .unwrap_or_else(|_| bridge_error_json("bridge-serialization-error")),
        Err(error) => {
            hbb_common::log::warn!("Invalid Corevia Desk bridge message: {}", error);
            bridge_error_json("invalid-bridge-message")
        }
    }
}

fn bridge_error_json(code: &str) -> String {
    serde_json::json!({
        "requestId": "native-bridge-error",
        "operation": "unknown",
        "surface": "nuxt",
        "profile": crate::corevia_acl::ProductProfile::current().as_str(),
        "status": "error",
        "error": {
            "code": code,
            "message": "The Corevia Desk bridge request was invalid."
        }
    })
    .to_string()
}

fn resolve_web_root() -> Result<PathBuf, String> {
    let mut candidates = Vec::new();
    if let Some(path) = std::env::var_os(WEB_ROOT_ENV) {
        candidates.extend(public_root_candidates(PathBuf::from(path)));
    }

    if let Ok(executable) = std::env::current_exe() {
        if let Some(parent) = executable.parent() {
            candidates.extend(public_root_candidates(parent.join("web")));
            candidates.extend(public_root_candidates(
                parent.join("..").join("Resources").join("web"),
            ));
        }
    }

    if let Ok(current_dir) = std::env::current_dir() {
        candidates.extend(public_root_candidates(current_dir.join("web")));
    }

    for candidate in &candidates {
        if candidate.join("index.html").is_file() {
            return candidate
                .canonicalize()
                .map_err(|error| format!("{} ({})", candidate.display(), error));
        }
    }

    Err(format!(
        "no generated Nuxt index.html found; checked {} locations. Set {} to the generated public directory",
        candidates.len(),
        WEB_ROOT_ENV
    ))
}

fn public_root_candidates(path: PathBuf) -> Vec<PathBuf> {
    vec![
        path.clone(),
        path.join("public"),
        path.join(".output").join("public"),
    ]
}

fn serve_asset(root: &Path, request: Request<Vec<u8>>) -> Response<Cow<'static, [u8]>> {
    let requested_path = request.uri().path();
    let relative_path = decode_path(requested_path).unwrap_or_else(|| "index.html".to_owned());
    let relative_path = relative_path.trim_start_matches('/');

    if let Some((connection_id, display)) = parse_frame_path(relative_path) {
        return match crate::webview_session::frame_bytes(connection_id, display) {
            Some(bytes) => asset_response(200, "image/png", bytes),
            None => asset_response(404, "text/plain; charset=utf-8", b"Frame not ready".to_vec()),
        };
    }

    let candidate = root.join(if relative_path.is_empty() {
        "index.html"
    } else {
        relative_path
    });

    let path = match safe_asset_path(root, &candidate) {
        Some(path) if path.is_file() => path,
        _ if should_fallback_to_spa(relative_path) => root.join("index.html"),
        _ => return asset_response(404, "text/plain; charset=utf-8", b"Not found".to_vec()),
    };

    match std::fs::read(&path) {
        Ok(bytes) => asset_response(200, content_type(&path), bytes),
        Err(error) => {
            hbb_common::log::warn!(
                "Corevia Desk asset read failed for {}: {}",
                path.display(),
                error
            );
            asset_response(
                500,
                "text/plain; charset=utf-8",
                b"Asset read failed".to_vec(),
            )
        }
    }
}

fn parse_frame_path(path: &str) -> Option<(i32, usize)> {
    let mut parts = path.split('/');
    if parts.next()? != "frames" {
        return None;
    }
    let connection_id = parts.next()?.parse::<i32>().ok()?;
    let display = parts.next()?.parse::<usize>().ok()?;
    if parts.next()? != "frame.png" || parts.next().is_some() {
        return None;
    }
    Some((connection_id, display))
}

fn safe_asset_path(root: &Path, candidate: &Path) -> Option<PathBuf> {
    let canonical_root = root.canonicalize().ok()?;
    let canonical_candidate = candidate.canonicalize().ok()?;
    canonical_candidate
        .starts_with(&canonical_root)
        .then_some(canonical_candidate)
}

fn should_fallback_to_spa(path: &str) -> bool {
    !path.rsplit('/').next().unwrap_or_default().contains('.')
}

fn decode_path(path: &str) -> Option<String> {
    let mut bytes = Vec::with_capacity(path.len());
    let raw = path.as_bytes();
    let mut index = 0;
    while index < raw.len() {
        if raw[index] == b'%' {
            if index + 2 >= raw.len() {
                return None;
            }
            let high = hex_value(raw[index + 1])?;
            let low = hex_value(raw[index + 2])?;
            bytes.push((high << 4) | low);
            index += 3;
        } else {
            bytes.push(raw[index]);
            index += 1;
        }
    }
    let decoded = String::from_utf8(bytes).ok()?;
    if decoded.contains('\0') || decoded.contains('\\') {
        return None;
    }
    Some(decoded)
}

fn hex_value(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}

fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("js") | Some("mjs") => "text/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("json") | Some("map") => "application/json; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("ico") => "image/x-icon",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("ttf") => "font/ttf",
        Some("wasm") => "application/wasm",
        _ => "application/octet-stream",
    }
}

fn asset_response(status: u16, content_type: &str, bytes: Vec<u8>) -> Response<Cow<'static, [u8]>> {
    Response::builder()
        .status(status)
        .header(CONTENT_TYPE, content_type)
        .header("Cache-Control", "no-cache")
        .body(Cow::Owned(bytes))
        .expect("valid Corevia Desk asset response")
}

#[cfg(test)]
mod tests {
    use super::{decode_path, dispatch_bridge_message, parse_frame_path, should_fallback_to_spa};
    use crate::corevia_bridge::{BridgeResponse, BridgeStatus};

    #[test]
    fn decodes_safe_asset_paths() {
        assert_eq!(
            decode_path("/_nuxt/main%20file.js"),
            Some("/_nuxt/main file.js".to_owned())
        );
        assert_eq!(decode_path("/..%2Fsecret"), Some("/../secret".to_owned()));
        assert_eq!(decode_path("/bad%2"), None);
        assert_eq!(decode_path("/bad%5Cpath"), None);
    }

    #[test]
    fn only_routes_without_file_extensions_use_spa_fallback() {
        assert!(should_fallback_to_spa("sessions"));
        assert!(should_fallback_to_spa("sessions/details"));
        assert!(!should_fallback_to_spa("_nuxt/app.js"));
    }

    #[test]
    fn parses_only_native_frame_paths() {
        assert_eq!(parse_frame_path("frames/1000000/0/frame.png"), Some((1000000, 0)));
        assert_eq!(parse_frame_path("frames/not-an-id/0/frame.png"), None);
        assert_eq!(parse_frame_path("frames/1000000/0/frame.jpg"), None);
    }

    #[test]
    fn native_bridge_forwards_requests_to_the_rust_acl() {
        let response = dispatch_bridge_message(
            r#"{"requestId":"native-smoke-1","operation":"get-status","payload":{}}"#,
        );
        let response: BridgeResponse = serde_json::from_str(&response).expect("bridge response");
        assert_eq!(response.request_id, "native-smoke-1");
        assert_eq!(response.status, BridgeStatus::Accepted);
        assert_eq!(response.surface, "nuxt");
        assert_eq!(
            response.data.expect("status data")["product"],
            crate::branding::APP_NAME
        );
    }

    #[test]
    fn native_bridge_returns_a_safe_error_for_invalid_messages() {
        let response = dispatch_bridge_message("not-json");
        let response: BridgeResponse = serde_json::from_str(&response).expect("bridge error");
        assert_eq!(response.status, BridgeStatus::Error);
        assert_eq!(
            response.error.expect("bridge error details").code,
            "invalid-bridge-message"
        );
    }
}
