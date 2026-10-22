//! The narrow request boundary used by the Nuxt view layer and future local
//! adapters.
//!
//! This module is the single, allowlisted boundary for the Nuxt view and MCP.
//! Reads and the operations that already have a native Rust hook are executed
//! here. Operations that need a platform-specific executor keep an explicit
//! hand-off in the response; they never become a generic command or endpoint
//! passthrough.

use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, Write},
    time::{SystemTime, UNIX_EPOCH},
};

use hbb_common::config::{keys, Config, LocalConfig, PeerConfig};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::Sha256;

use crate::corevia_acl::{
    authorize, authorize_connection_permission, is_customer_support_setting,
    is_known_connection_permission, AuthorizationRequest, Capability, DenialReason, Surface,
};

const MAX_REQUEST_ID_LENGTH: usize = 128;
const COREVIA_LAST_CONNECTED_AT_OPTION: &str = "corevia-last-connected-at";
const MCP_APPROVAL_KEY_ENV: &str = "COREVIA_DESK_MCP_APPROVAL_KEY";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BridgeStatus {
    Accepted,
    Denied,
    Unsupported,
    Error,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BridgeRequest {
    #[serde(rename = "requestId")]
    pub request_id: String,
    pub operation: String,
    #[serde(default = "empty_payload")]
    pub payload: Value,
    #[serde(default, rename = "approvalReference")]
    pub approval_reference: Option<String>,
    #[serde(default, rename = "approvalProof")]
    pub approval_proof: Option<String>,
}

fn empty_payload() -> Value {
    json!({})
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BridgeError {
    pub code: String,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum BridgeSessionState {
    Pending,
    Active,
    Closed,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BridgeSession {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "remoteId")]
    pub remote_id: String,
    pub state: BridgeSessionState,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum BridgeEventKind {
    RequestAccepted,
    RequestDenied,
    SessionOpened,
    SessionClosed,
    PermissionChanged,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BridgeEvent {
    #[serde(rename = "eventId")]
    pub event_id: String,
    pub kind: BridgeEventKind,
    #[serde(rename = "sessionId", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    pub data: Value,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BridgeResponse {
    #[serde(rename = "requestId")]
    pub request_id: String,
    pub operation: String,
    pub surface: String,
    pub profile: String,
    pub status: BridgeStatus,
    #[serde(skip_serializing_if = "Option::is_none", rename = "auditId")]
    pub audit_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<BridgeEvent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<BridgeError>,
}

/// Authorize a request arriving from Nuxt.
pub fn dispatch(request: BridgeRequest) -> BridgeResponse {
    dispatch_for_surface(Surface::Nuxt, request)
}

/// Authorize the same explicit operation catalog for another trusted local
/// adapter, such as the MCP server. The adapter chooses the surface; the
/// capability decision still belongs to `corevia_acl`.
pub fn dispatch_for_surface(surface: Surface, request: BridgeRequest) -> BridgeResponse {
    let approval_key = std::env::var(MCP_APPROVAL_KEY_ENV)
        .ok()
        .and_then(|key| hex::decode(key).ok());
    let mut used_approval_references = HashSet::new();
    dispatch_for_surface_with_approval_context(
        surface,
        request,
        approval_key.as_deref(),
        &mut used_approval_references,
    )
}

fn dispatch_for_surface_with_approval_context(
    surface: Surface,
    request: BridgeRequest,
    approval_key: Option<&[u8]>,
    used_approval_references: &mut HashSet<String>,
) -> BridgeResponse {
    // Tests and the MCP child process can enter this module without going
    // through the normal desktop bootstrap. Make the product defaults visible
    // in those paths as well.
    crate::branding::initialize();

    let request_id = request.request_id.clone();
    let operation = request.operation.clone();
    let profile = crate::corevia_acl::ProductProfile::current();

    if !is_valid_request_id(&request_id) {
        return error_response(
            request_id,
            operation,
            surface,
            "invalid-request-id",
            "The request id is missing or invalid.",
        );
    }
    if !request.payload.is_object() {
        return error_response(
            request_id,
            operation,
            surface,
            "invalid-payload",
            "The bridge payload must be a JSON object.",
        );
    }

    let capability = match resolve_operation(&operation, &request.payload) {
        OperationResolution::Capability(capability) => capability,
        OperationResolution::Unsupported => {
            return BridgeResponse {
                request_id,
                operation,
                surface: surface.as_str().to_owned(),
                profile: profile.as_str().to_owned(),
                status: BridgeStatus::Unsupported,
                audit_id: None,
                data: None,
                events: None,
                error: Some(BridgeError {
                    code: "operation-not-available".to_owned(),
                    message: "This named operation is not part of the Corevia Desk bridge.".to_owned(),
                }),
            };
        }
        OperationResolution::Denied(reason) => {
            return denied_response(request_id, operation, surface, reason);
        }
        OperationResolution::InvalidPayload(message) => {
            return error_response(
                request_id,
                operation,
                surface,
                "invalid-payload",
                message,
            );
        }
    };

    let mcp_approval_verified = surface == Surface::Mcp
        && verify_mcp_approval(&request, approval_key)
        && request
            .approval_reference
            .as_ref()
            .is_some_and(|reference| used_approval_references.insert(reference.clone()));

    // The temporary password is a local UI disclosure. It is deliberately not
    // a shared MCP capability and must never become a remote secret endpoint.
    if operation == "read-temporary-password" && surface != Surface::Nuxt {
        return denied_response(
            request_id,
            operation,
            surface,
            DenialReason::CapabilityNotAvailable,
        );
    }

    // Screen-recording consent belongs to this app instance. Its status and
    // request flow are local UI only; MCP can neither inspect nor request it.
    if matches!(
        operation.as_str(),
        "read-screen-capture-status" | "request-screen-capture-access"
    ) && surface != Surface::Nuxt
    {
        return denied_response(
            request_id,
            operation,
            surface,
            DenialReason::CapabilityNotAvailable,
        );
    }

    // The live terminal stream is an interactive desktop surface. Do not let
    // generic adapters read or write its contents through guessed operation
    // names; MCP remains limited to its separately reviewed catalog.
    if matches!(
        operation.as_str(),
        "read-terminal"
            | "send-terminal-input"
            | "close-terminal"
            | "read-file-transfer"
            | "read-transfer-chunk"
            | "cleanup-file-transfer"
            | "read-remote-folder"
            | "send-file-to-remote"
            | "download-remote-file"
    ) && surface != Surface::Nuxt
    {
        return denied_response(
            request_id,
            operation,
            surface,
            DenialReason::CapabilityNotAvailable,
        );
    }

    let authorization = if operation == "change-connection-permission" {
        if mcp_approval_verified {
            crate::corevia_acl::authorize_connection_permission_with_verified_mcp_approval(
                required_string(&request.payload, "permission").unwrap_or_default(),
                request.approval_reference.as_deref().unwrap_or_default(),
            )
        } else {
            authorize_connection_permission(
                surface,
                required_string(&request.payload, "permission").unwrap_or_default(),
                request.approval_reference.as_deref().unwrap_or_default(),
            )
        }
    } else {
        match request.approval_reference.as_deref() {
            Some(approval_reference) if mcp_approval_verified => authorize(
                AuthorizationRequest::with_verified_mcp_approval(capability, approval_reference),
            ),
            Some(approval_reference) => authorize(AuthorizationRequest::with_approval(
                surface,
                capability,
                approval_reference,
            )),
            None => authorize(AuthorizationRequest::new(surface, capability)),
        }
    };

    match authorization {
        Ok(receipt) => match execute_authorized(
            surface,
            &operation,
            &request.payload,
            request.approval_reference.as_deref(),
        ) {
            Ok(mut data) => {
                if let Some(object) = data.as_object_mut() {
                    object.insert("authorized".to_owned(), json!(true));
                }
                BridgeResponse {
                    request_id,
                    operation,
                    surface: surface.as_str().to_owned(),
                    profile: receipt.profile.as_str().to_owned(),
                    status: BridgeStatus::Accepted,
                    audit_id: Some(receipt.audit_id),
                    data: Some(data),
                    events: None,
                    error: None,
                }
            }
            Err(error) => BridgeResponse {
                request_id,
                operation,
                surface: surface.as_str().to_owned(),
                profile: receipt.profile.as_str().to_owned(),
                status: BridgeStatus::Error,
                audit_id: Some(receipt.audit_id),
                data: None,
                events: None,
                error: Some(error),
            },
        },
        Err(error) => denied_response(request_id, operation, surface, error.reason),
    }
}

fn verify_mcp_approval(request: &BridgeRequest, key: Option<&[u8]>) -> bool {
    let (Some(key), Some(reference), Some(proof)) = (
        key,
        request.approval_reference.as_deref(),
        request.approval_proof.as_deref(),
    ) else {
        return false;
    };
    if key.len() != 32 {
        return false;
    }
    let Ok(proof) = hex::decode(proof) else {
        return false;
    };
    let Ok(payload) = serde_json::to_string(&request.payload) else {
        return false;
    };
    let Ok(mut mac) = Hmac::<Sha256>::new_from_slice(key) else {
        return false;
    };
    mac.update(reference.as_bytes());
    mac.update(&[0]);
    mac.update(request.operation.as_bytes());
    mac.update(&[0]);
    mac.update(payload.as_bytes());
    mac.verify_slice(&proof).is_ok()
}

fn execute_authorized(
    surface: Surface,
    operation: &str,
    payload: &Value,
    approval_reference: Option<&str>,
) -> Result<Value, BridgeError> {
    match operation {
        "get-status" => Ok(status_snapshot()),
        "read-screen-capture-status" => {
            #[cfg(target_os = "macos")]
            {
                let granted = scrap::screen_capture_access_granted();
                return Ok(json!({
                    "supported": true,
                    "granted": granted,
                    "needsRestartAfterGrant": true
                }));
            }
            #[cfg(not(target_os = "macos"))]
            Ok(json!({
                "supported": false,
                "granted": Value::Null,
                "needsRestartAfterGrant": false
            }))
        }
        "request-screen-capture-access" => {
            #[cfg(target_os = "macos")]
            {
                let granted = scrap::request_screen_capture_access();
                return Ok(json!({
                    "supported": granted.is_some(),
                    "granted": granted,
                    "needsRestartAfterGrant": true
                }));
            }
            #[cfg(not(target_os = "macos"))]
            Ok(json!({
                "supported": false,
                "granted": Value::Null,
                "needsRestartAfterGrant": false
            }))
        }
        "read-settings" => Ok(settings_snapshot()),
        "read-identity" => Ok(identity_snapshot()),
        "read-temporary-password" => {
            #[cfg(not(test))]
            {
                return Ok(json!({
                    "temporaryPassword": crate::ui_interface::temporary_password()
                }));
            }
            #[cfg(test)]
            Ok(json!({ "temporaryPassword": "" }))
        }
        "read-devices" => Ok(json!({ "devices": devices_snapshot() })),
        "read-sessions" => {
            let (active, history) = sessions_snapshot();
            Ok(json!({ "active": active, "history": history }))
        }
        "read-terminal" => {
            let id = connection_id(payload)?;
            #[cfg(not(test))]
            if let Some(snapshot) = crate::webview_session::terminal_snapshot(id) {
                return Ok(snapshot);
            }
            Ok(json!({
                "open": false,
                "supported": false,
                "status": "unavailable",
                "output": "",
                "error": "The terminal is not available for this session.",
                "exitCode": Value::Null
            }))
        }
        "read-file-transfer" => {
            let id = connection_id(payload)?;
            #[cfg(not(test))]
            if let Some(snapshot) = crate::webview_session::transfer_snapshot(id) {
                return Ok(snapshot);
            }
            Ok(json!({
                "status": "unavailable",
                "direction": "",
                "name": "",
                "error": "File transfer is not available for this session.",
                "bytes": 0,
                "size": 0,
                "speed": 0,
                "remoteDirectory": "",
                "remoteDirectoryStatus": "unavailable",
                "remoteDirectoryError": "",
                "entries": []
            }))
        }
        "read-session-frame" => {
            let id = connection_id(payload)?;
            #[cfg(not(any(
                target_os = "android",
                target_os = "ios",
                feature = "flutter"
            )))]
            if let Some(frame) = crate::webview_session::frame_descriptor(id) {
                Ok(json!({
                    "available": true,
                    "connectionId": id.to_string(),
                    "display": frame.display,
                    "sequence": frame.sequence,
                    "width": frame.width,
                    "height": frame.height,
                    "frameUrl": format!(
                        "corevia://localhost/frames/{}/{}/frame.png?sequence={}",
                        id, frame.display, frame.sequence
                    )
                }))
            } else {
                Ok(json!({
                    "available": false,
                    "connectionId": id.to_string()
                }))
            }
            #[cfg(any(target_os = "android", target_os = "ios", feature = "flutter"))]
            Ok(json!({
                "available": false,
                "connectionId": id.to_string()
            }))
        }
        "write-customer-setting" | "write-global-setting" => {
            let key = required_string(payload, "key").ok_or_else(|| invalid_field("key"))?;
            let value = scalar_to_config_value(
                payload
                    .get("value")
                    .ok_or_else(|| invalid_field("value"))?,
            )?;
            write_setting(key, value.clone());
            Ok(json!({
                "executed": true,
                "next": "completed",
                "key": key,
                "value": if is_sensitive_setting_key(key) { json!(null) } else { json!(value) }
            }))
        }
        "start-connection" => {
            let requested_remote_id = required_string(payload, "remoteId")
                .ok_or_else(|| invalid_field("remoteId"))?;
            let remote_id = crate::ui_interface::handle_relay_id(requested_remote_id);
            let force_relay = remote_id != requested_remote_id;
            if !crate::common::is_valid_untrusted_peer_id(remote_id) {
                return Err(BridgeError {
                    code: "invalid-peer-id".to_owned(),
                    message: "The remote device identifier is invalid.".to_owned(),
                });
            }
            #[cfg(not(test))]
            {
                let connection_id = crate::webview_session::start_connection(remote_id.to_owned(), force_relay)
                    .ok_or_else(|| BridgeError {
                        code: "connection-worker-start-failed".to_owned(),
                        message: "The native Rust connection worker could not start.".to_owned(),
                    })?;
                return Ok(json!({
                    "executed": true,
                    "next": "connecting",
                    "connectionId": connection_id
                }));
            }
            #[cfg(test)]
            Ok(executed(true))
        }
        "approve-connection" => {
            let id = connection_id(payload)?;
            let client = clients_snapshot().into_iter().find(|client| {
                client.get("id").and_then(Value::as_i64) == Some(id as i64)
            });
            let Some(client) = client else {
                return Ok(executed_with_reason(false, "connection-not-found"));
            };
            if !client_is_incoming(&client) {
                return Ok(executed_with_reason(false, "outgoing-session-does-not-need-approval"));
            }
            crate::ui_cm_interface::authorize_from(
                surface,
                id,
                approval_reference.unwrap_or_default(),
            );
            Ok(executed(true))
        }
        "close-session" => {
            let id = connection_id(payload)?;
            #[cfg(not(test))]
            if crate::webview_session::close(id) {
                return Ok(executed(true));
            }
            Ok(executed(crate::ui_cm_interface::close(id)))
        }
        "change-connection-permission" => {
            let id = connection_id(payload)?;
            let permission = required_string(payload, "permission")
                .ok_or_else(|| invalid_field("permission"))?;
            let enabled = payload
                .get("enabled")
                .and_then(Value::as_bool)
                .ok_or_else(|| invalid_field("enabled"))?;
            #[cfg(not(test))]
            if crate::webview_session::change_permission(id, permission, enabled) {
                return Ok(executed(true));
            }
            Ok(executed(crate::ui_cm_interface::switch_permission_from(
                surface,
                id,
                permission.to_owned(),
                enabled,
                approval_reference.unwrap_or_default(),
            )))
        }
        "input-control" => {
            let id = connection_id(payload)?;
            let enabled = payload
                .get("enabled")
                .and_then(Value::as_bool)
                .unwrap_or(true);
            #[cfg(not(test))]
            if crate::webview_session::change_permission(id, "keyboard", enabled) {
                return Ok(executed(true));
            }
            Ok(executed(crate::ui_cm_interface::switch_permission_from(
                surface,
                id,
                "keyboard".to_owned(),
                enabled,
                approval_reference.unwrap_or_default(),
            )))
        }
        "send-mouse" => {
            let id = connection_id(payload)?;
            let mask = required_i32(payload, "mask")?;
            let x = required_i32(payload, "x")?;
            let y = required_i32(payload, "y")?;
            #[cfg(not(any(
                target_os = "android",
                target_os = "ios",
                feature = "flutter"
            )))]
            let sent = crate::webview_session::send_mouse(
                id,
                mask,
                x,
                y,
                payload.get("alt").and_then(Value::as_bool).unwrap_or(false),
                payload.get("ctrl").and_then(Value::as_bool).unwrap_or(false),
                payload.get("shift").and_then(Value::as_bool).unwrap_or(false),
                payload
                    .get("command")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            );
            #[cfg(any(target_os = "android", target_os = "ios", feature = "flutter"))]
            let sent = false;
            Ok(executed(sent))
        }
        "send-key" => {
            let id = connection_id(payload)?;
            let name = required_string(payload, "name").ok_or_else(|| invalid_field("name"))?;
            let down = payload
                .get("down")
                .and_then(Value::as_bool)
                .ok_or_else(|| invalid_field("down"))?;
            #[cfg(not(any(
                target_os = "android",
                target_os = "ios",
                feature = "flutter"
            )))]
            let sent = crate::webview_session::send_key(
                id,
                name,
                down,
                payload.get("press").and_then(Value::as_bool).unwrap_or(false),
                payload.get("alt").and_then(Value::as_bool).unwrap_or(false),
                payload.get("ctrl").and_then(Value::as_bool).unwrap_or(false),
                payload.get("shift").and_then(Value::as_bool).unwrap_or(false),
                payload
                    .get("command")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            );
            #[cfg(any(target_os = "android", target_os = "ios", feature = "flutter"))]
            let sent = false;
            Ok(executed(sent))
        }
        "use-file-transfer" => {
            let id = connection_id(payload)?;
            #[cfg(not(test))]
            if crate::webview_session::change_permission(id, "file", true) {
                return Ok(executed(true));
            }
            Ok(executed(crate::ui_cm_interface::switch_permission_from(
                surface,
                id,
                "file".to_owned(),
                true,
                approval_reference.unwrap_or_default(),
            )))
        }
        "read-remote-folder" => {
            let id = connection_id(payload)?;
            let path = required_string(payload, "path").ok_or_else(|| invalid_field("path"))?;
            #[cfg(not(test))]
            let started = crate::webview_session::read_remote_folder(id, path);
            #[cfg(test)]
            let started = false;
            Ok(executed_with_reason(
                started,
                if started { "remote-folder-request-sent" } else { "remote-folder-unavailable-or-invalid" },
            ))
        }
        "send-file-to-remote" => {
            let id = connection_id(payload)?;
            let name = required_string(payload, "fileName")
                .ok_or_else(|| invalid_field("fileName"))?;
            let directory = required_string(payload, "remoteDirectory")
                .ok_or_else(|| invalid_field("remoteDirectory"))?;
            let content = required_string(payload, "contentBase64")
                .ok_or_else(|| invalid_field("contentBase64"))?;
            #[cfg(not(test))]
            let started = crate::webview_session::upload_file(id, name, directory, content);
            #[cfg(test)]
            let started = false;
            Ok(executed_with_reason(
                started,
                if started { "upload-started" } else { "upload-unavailable-or-invalid" },
            ))
        }
        "download-remote-file" => {
            let id = connection_id(payload)?;
            let name = required_string(payload, "fileName")
                .ok_or_else(|| invalid_field("fileName"))?;
            #[cfg(not(test))]
            let started = crate::webview_session::download_remote_file(id, name);
            #[cfg(test)]
            let started = false;
            Ok(executed_with_reason(
                started,
                if started { "download-started" } else { "download-unavailable-or-not-listed" },
            ))
        }
        "read-transfer-chunk" => {
            let id = connection_id(payload)?;
            let offset = payload.get("offset").and_then(Value::as_u64)
                .ok_or_else(|| invalid_field("offset"))?;
            let chunk_size = payload.get("maxBytes").and_then(Value::as_u64)
                .and_then(|value| usize::try_from(value).ok())
                .ok_or_else(|| invalid_field("maxBytes"))?;
            #[cfg(not(test))]
            if let Some(chunk) = crate::webview_session::read_transfer_chunk(id, offset, chunk_size) {
                return Ok(chunk);
            }
            Ok(json!({ "error": "Transfer chunk is not available." }))
        }
        "cleanup-file-transfer" => {
            let id = connection_id(payload)?;
            #[cfg(not(test))]
            let cleaned = crate::webview_session::cleanup_transfer(id);
            #[cfg(test)]
            let cleaned = false;
            Ok(executed(cleaned))
        }
        "record-session" => {
            let id = connection_id(payload)?;
            let enabled = payload
                .get("enabled")
                .and_then(Value::as_bool)
                .ok_or_else(|| invalid_field("enabled"))?;
            #[cfg(not(test))]
            if crate::webview_session::record_screen(id, enabled) {
                return Ok(executed(true));
            }
            Ok(executed(crate::ui_cm_interface::switch_permission_from(
                surface,
                id,
                "recording".to_owned(),
                enabled,
                approval_reference.unwrap_or_default(),
            )))
        }
        "toggle-privacy-mode" => {
            let id = connection_id(payload)?;
            let enabled = payload
                .get("enabled")
                .and_then(Value::as_bool)
                .ok_or_else(|| invalid_field("enabled"))?;
            #[cfg(not(test))]
            if crate::webview_session::toggle_privacy_mode(id, enabled) {
                return Ok(executed(true));
            }
            Ok(executed(crate::ui_cm_interface::switch_permission_from(
                surface,
                id,
                "privacy_mode".to_owned(),
                enabled,
                approval_reference.unwrap_or_default(),
            )))
        }
        "send-chat" => {
            let id = connection_id(payload)?;
            let message = required_string(payload, "message")
                .ok_or_else(|| invalid_field("message"))?;
            #[cfg(not(test))]
            if crate::webview_session::send_chat(id, message.to_owned()) {
                return Ok(executed(true));
            }
            Ok(executed(crate::ui_cm_interface::send_chat_from(
                surface,
                id,
                message.to_owned(),
            )))
        }
        "switch-display" => {
            let id = connection_id(payload)?;
            let display = required_i32(payload, "display")?;
            #[cfg(not(test))]
            if crate::webview_session::switch_display(id, display) {
                return Ok(executed(true));
            }
            Ok(executed(false))
        }
        "refresh-video" => {
            let id = connection_id(payload)?;
            #[cfg(not(test))]
            if crate::webview_session::refresh_video(id) {
                return Ok(executed(true));
            }
            Ok(executed(false))
        }
        "set-session-quality" => {
            let id = connection_id(payload)?;
            let quality = required_string(payload, "quality")
                .ok_or_else(|| invalid_field("quality"))?;
            #[cfg(not(test))]
            if crate::webview_session::set_image_quality(id, quality.to_owned()) {
                return Ok(executed(true));
            }
            Ok(executed(false))
        }
        "set-session-scroll" => {
            let id = connection_id(payload)?;
            let style = required_string(payload, "style").ok_or_else(|| invalid_field("style"))?;
            #[cfg(not(test))]
            if crate::webview_session::set_scroll_style(id, style) {
                return Ok(executed(true));
            }
            Ok(executed(false))
        }
        "use-terminal" => {
            let id = connection_id(payload)?;
            #[cfg(not(test))]
            if crate::webview_session::open_terminal(id) {
                return Ok(executed_with_reason(true, "terminal-open-request-sent"));
            }
            Ok(executed_with_reason(false, "terminal-unavailable-or-unsupported"))
        }
        "close-terminal" => {
            let id = connection_id(payload)?;
            #[cfg(not(test))]
            if crate::webview_session::close_terminal(id) {
                return Ok(executed(true));
            }
            Ok(executed_with_reason(false, "terminal-not-open"))
        }
        "send-terminal-input" => {
            let id = connection_id(payload)?;
            let input = required_string(payload, "input").ok_or_else(|| invalid_field("input"))?;
            #[cfg(not(test))]
            if crate::webview_session::send_terminal_input(id, input) {
                return Ok(executed(true));
            }
            Ok(executed_with_reason(false, "terminal-not-open-or-input-invalid"))
        }
        "use-voice-call" => {
            let id = connection_id(payload)?;
            let enabled = payload.get("enabled").and_then(Value::as_bool).unwrap_or(true);
            #[cfg(not(test))]
            let executed_value = if enabled {
                crate::webview_session::request_voice_call(id)
            } else {
                crate::webview_session::close_voice_call(id)
            };
            #[cfg(test)]
            let executed_value = false;
            Ok(executed(executed_value))
        }
        "restart-remote-device" => {
            let id = connection_id(payload)?;
            #[cfg(not(test))]
            if crate::webview_session::restart_remote_device(id) {
                return Ok(executed(true));
            }
            Ok(executed(false))
        }
        "elevate-session" => {
            let id = connection_id(payload)?;
            #[cfg(not(test))]
            if crate::webview_session::elevate(id) {
                return Ok(executed(true));
            }
            if crate::ui_cm_interface::can_elevate() {
                crate::ui_cm_interface::elevate_portable(id);
                Ok(executed(true))
            } else {
                Ok(executed_with_reason(false, "platform-elevation-not-available"))
            }
        }
        "use-port-forward" => {
            let id = connection_id(payload)?;
            let local_port = required_i32(payload, "localPort")?;
            let remote_port = required_i32(payload, "remotePort")?;
            let remote_host = required_string(payload, "remoteHost").unwrap_or("127.0.0.1");
            #[cfg(not(test))]
            if crate::webview_session::add_port_forward(
                id,
                local_port,
                remote_host.to_owned(),
                remote_port,
            ) {
                return Ok(executed(true));
            }
            Ok(executed(false))
        }
        "manage-peers" => execute_manage_peers(payload),
        // These operations still need a native UI-specific completion flow.
        // Keep the result explicit rather than reporting a false success.
        "use-camera" | "use-printer" | "manage-service" => Ok(json!({
            "executed": false,
            "next": "native-executor",
            "reason": "platform-executor-required"
        })),
        _ => Ok(executed(false)),
    }
}

fn executed(value: bool) -> Value {
    json!({
        "executed": value,
        "next": "completed"
    })
}

fn executed_with_reason(value: bool, reason: &str) -> Value {
    json!({
        "executed": value,
        "next": "completed",
        "reason": reason
    })
}

fn invalid_field(field: &str) -> BridgeError {
    BridgeError {
        code: "invalid-field-type".to_owned(),
        message: format!("The '{}' field has an invalid value.", field),
    }
}

fn connection_id(payload: &Value) -> Result<i32, BridgeError> {
    let value = required_string(payload, "connectionId").ok_or_else(|| invalid_field("connectionId"))?;
    value.parse::<i32>().map_err(|_| BridgeError {
        code: "invalid-connection-id".to_owned(),
        message: "The connection identifier is invalid.".to_owned(),
    })
}

fn required_i32(payload: &Value, key: &str) -> Result<i32, BridgeError> {
    payload
        .get(key)
        .and_then(Value::as_i64)
        .and_then(|value| i32::try_from(value).ok())
        .ok_or_else(|| invalid_field(key))
}

fn scalar_to_config_value(value: &Value) -> Result<String, BridgeError> {
    match value {
        Value::Null => Ok(String::new()),
        Value::Bool(value) => Ok(if *value { "Y" } else { "N" }.to_owned()),
        Value::Number(value) => Ok(value.to_string()),
        Value::String(value) => Ok(value.clone()),
        Value::Array(_) | Value::Object(_) => Err(BridgeError {
            code: "invalid-setting-value".to_owned(),
            message: "A setting value must be a simple value.".to_owned(),
        }),
    }
}

fn write_setting(key: &str, value: String) {
    if is_local_setting_key(key) {
        crate::ui_interface::set_local_option(key.to_owned(), value);
    } else {
        crate::ui_interface::set_option(key.to_owned(), value);
        crate::ui_interface::refresh_options();
    }
}

fn is_local_setting_key(key: &str) -> bool {
    keys::KEYS_LOCAL_SETTINGS.contains(&key)
}

fn is_known_global_setting(key: &str) -> bool {
    key != "stop-service"
        && (keys::KEYS_SETTINGS.contains(&key) || keys::KEYS_DISPLAY_SETTINGS.contains(&key))
}

fn is_sensitive_setting_key(key: &str) -> bool {
    let key = key.to_ascii_lowercase();
    key == keys::OPTION_KEY
        || key.contains("password")
        || key.contains("secret")
        || key.contains("token")
}

fn clients_snapshot() -> Vec<Value> {
    let mut clients: Vec<Value> =
        serde_json::from_str(crate::ui_cm_interface::get_clients_state().as_str())
            .unwrap_or_default();
    let legacy_count = clients.len();
    // The legacy connection manager only owns inbound requests.  Give its
    // records an explicit source marker before merging them with the native
    // WebView outbound-session registry.
    for client in &mut clients {
        if let Some(object) = client.as_object_mut() {
            object.insert("incoming".to_owned(), json!(true));
        }
    }
    #[cfg(not(any(target_os = "android", target_os = "ios", feature = "flutter")))]
    clients.extend(crate::webview_session::snapshot_clients());
    let raw_count = clients.len();
    let clients = deduplicate_clients(clients);
    hbb_common::log::debug!(
        "Corevia bridge client snapshot: legacy_count={}, raw_count={}, unique_count={}",
        legacy_count,
        raw_count,
        clients.len()
    );
    clients
}

fn client_is_incoming(client: &Value) -> bool {
    client
        .get("incoming")
        .and_then(Value::as_bool)
        .unwrap_or(true)
}

/// The legacy connection manager and the native WebView session registry can
/// briefly report the same connection at the same time. Keep one record per
/// real connection so the UI does not duplicate a machine in active sessions.
/// A live record wins over a stale closed/failed record; records without a
/// stable connection id are intentionally kept because collapsing them could
/// hide two legitimate requests from the same peer.
fn deduplicate_clients(clients: Vec<Value>) -> Vec<Value> {
    let mut positions = HashMap::<i64, usize>::new();
    let mut unique = Vec::with_capacity(clients.len());

    for client in clients {
        let id = client
            .get("id")
            .and_then(Value::as_i64)
            .filter(|id| *id != 0);

        let Some(id) = id else {
            unique.push(client);
            continue;
        };

        if let Some(position) = positions.get(&id).copied() {
            if client_preference(&client) > client_preference(&unique[position]) {
                unique[position] = client;
            }
        } else {
            positions.insert(id, unique.len());
            unique.push(client);
        }
    }

    unique
}

fn client_preference(client: &Value) -> (u8, u8, u8) {
    let disconnected = client
        .get("disconnected")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let authorized = client
        .get("authorized")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let state = match client.get("state").and_then(Value::as_str) {
        Some("active") => 4,
        Some("pending") => 3,
        Some("closed") => 2,
        Some("failed") => 1,
        _ if !disconnected && authorized => 4,
        _ if !disconnected => 3,
        _ => 0,
    };
    (state, u8::from(authorized), u8::from(!disconnected))
}

fn identity_snapshot() -> Value {
    json!({
        "localId": Config::get_id(),
        "passwordConfigured": Config::has_permanent_password(),
        "temporaryPasswordAvailable": !crate::ui_interface::temporary_password().is_empty()
    })
}

fn settings_snapshot() -> Value {
    let profile = crate::corevia_acl::ProductProfile::current();
    let mut values = serde_json::Map::new();

    for (key, value) in Config::get_options() {
        if is_sensitive_setting_key(&key)
            || (profile == crate::corevia_acl::ProductProfile::CustomerSupport
                && !is_customer_support_setting(&key))
        {
            continue;
        }
        values.insert(key, json!(value));
    }
    for key in keys::KEYS_LOCAL_SETTINGS {
        if profile == crate::corevia_acl::ProductProfile::CustomerSupport
            && !is_customer_support_setting(key)
        {
            continue;
        }
        if is_sensitive_setting_key(key) {
            continue;
        }
        values.insert((*key).to_owned(), json!(LocalConfig::get_option(key)));
    }

    json!({
        "values": values,
        "network": {
            "idServer": Config::get_option(keys::OPTION_CUSTOM_RENDEZVOUS_SERVER),
            "relayServer": Config::get_option(keys::OPTION_RELAY_SERVER),
            "apiServer": Config::get_option(keys::OPTION_API_SERVER),
            "keyConfigured": !Config::get_option(keys::OPTION_KEY).is_empty()
        }
    })
}

fn status_snapshot() -> Value {
    let (active, _) = sessions_snapshot();
    let pending = clients_snapshot()
        .iter()
        .filter(|client| {
            client.get("authorized").and_then(Value::as_bool) == Some(false)
                && client.get("disconnected").and_then(Value::as_bool) == Some(false)
        })
        .count();
    let devices = devices_snapshot();
    let options = Config::get_options();
    json!({
        "product": crate::branding::APP_NAME,
        "version": crate::VERSION,
        "profile": crate::corevia_acl::ProductProfile::current().as_str(),
        "platform": std::env::consts::OS,
        "localId": Config::get_id(),
        "networkConfigured": !options.get(keys::OPTION_CUSTOM_RENDEZVOUS_SERVER).map(String::is_empty).unwrap_or(true)
            && !options.get(keys::OPTION_RELAY_SERVER).map(String::is_empty).unwrap_or(true),
        "idServer": options.get(keys::OPTION_CUSTOM_RENDEZVOUS_SERVER).cloned().unwrap_or_default(),
        "relayServer": options.get(keys::OPTION_RELAY_SERVER).cloned().unwrap_or_default(),
        "apiServer": options.get(keys::OPTION_API_SERVER).cloned().unwrap_or_default(),
        "keyConfigured": !options.get(keys::OPTION_KEY).map(String::is_empty).unwrap_or(true),
        "activeSessionCount": active.len(),
        "pendingSessionCount": pending,
        "savedDeviceCount": devices.len()
    })
}

fn clients_peer_metadata(peer_id: &str) -> (String, String) {
    if peer_id.is_empty() {
        return (String::new(), String::new());
    }
    let peer = PeerConfig::load(peer_id);
    (
        peer.info.hostname,
        peer.info.platform,
    )
}

fn sessions_snapshot() -> (Vec<Value>, Vec<Value>) {
    let mut active = Vec::new();
    let mut history = Vec::new();
    let mut record_states = Vec::new();

    for client in clients_snapshot() {
            let id = client
                .get("id")
                .and_then(Value::as_i64)
                .unwrap_or_default();
            let remote_id = client
                .get("peer_id")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_owned();
            let (peer_name, platform) = clients_peer_metadata(&remote_id);
            let fallback_name = nonempty_or(Some(peer_name.as_str()), remote_id.as_str());
            let name = nonempty_or(
                client.get("name").and_then(Value::as_str),
                &fallback_name,
            );
            let disconnected = client
                .get("disconnected")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let authorized = client
                .get("authorized")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let state = match client.get("state").and_then(Value::as_str) {
                Some("pending") => "pending",
                Some("active") => "active",
                Some("closed") => "closed",
                Some("failed") => "failed",
                _ if disconnected => "closed",
                _ if authorized => "active",
                _ => "pending",
            };
            let direct = client.get("direct").and_then(Value::as_bool);
            let secured = client.get("secured").and_then(Value::as_bool);
            let incoming = client_is_incoming(&client);
            let snapshot = json!({
                "sessionId": id.to_string(),
                "remoteId": remote_id,
                "name": name,
                "platform": if platform.is_empty() { "Unknown" } else { platform.as_str() },
                "address": if remote_id.is_empty() { name.as_str() } else { remote_id.as_str() },
                "state": state,
                "authorized": authorized,
                "disconnected": disconnected,
                "incoming": incoming,
                "direct": direct,
                "secured": secured,
                "permissions": session_permissions(&client),
                "features": {
                    "fileTransfer": client.get("is_file_transfer").and_then(Value::as_bool).unwrap_or(false),
                    "camera": client.get("is_view_camera").and_then(Value::as_bool).unwrap_or(false),
                    "terminal": client.get("is_terminal").and_then(Value::as_bool).unwrap_or(false),
                    "terminalSupported": client.get("terminal_supported").and_then(Value::as_bool).unwrap_or(false),
                    "terminalStatus": client.get("terminal_status").and_then(Value::as_str).unwrap_or("unavailable"),
                    "portForward": client.get("port_forward").and_then(Value::as_str).unwrap_or_default(),
                    "voiceCall": client.get("in_voice_call").and_then(Value::as_bool).unwrap_or(false)
                },
                "endedAt": client.get("endedAt").cloned().unwrap_or(Value::Null),
                "error": client.get("error").cloned().unwrap_or(Value::Null)
            });

            record_states.push(format!("{}:{}:{}", id, remote_id, state));

            if matches!(state, "closed" | "failed") || disconnected {
                history.push(snapshot);
            } else {
                active.push(snapshot);
            }
    }

    history.sort_by(compare_session_history_by_end);

    hbb_common::log::debug!(
        "Corevia bridge sessions snapshot: active_count={}, history_count={}",
        active.len(),
        history.len()
    );
    hbb_common::log::debug!(
        "Corevia bridge session records: {}",
        record_states.join(",")
    );
    (active, history)
}

fn session_permissions(client: &Value) -> Value {
    json!({
        "keyboard": client.get("keyboard").and_then(Value::as_bool).unwrap_or(false),
        "clipboard": client.get("clipboard").and_then(Value::as_bool).unwrap_or(false),
        "audio": client.get("audio").and_then(Value::as_bool).unwrap_or(false),
        "file": client.get("file").and_then(Value::as_bool).unwrap_or(false),
        "restart": client.get("restart").and_then(Value::as_bool).unwrap_or(false),
        "recording": client.get("recording").and_then(Value::as_bool).unwrap_or(false),
        "block_input": client.get("block_input").and_then(Value::as_bool).unwrap_or(false),
        "privacy_mode": client.get("privacy_mode").and_then(Value::as_bool).unwrap_or(false),
        "view_only": client.get("view_only").and_then(Value::as_bool).unwrap_or(false),
        "show_remote_cursor": client.get("show_remote_cursor").and_then(Value::as_bool).unwrap_or(false)
    })
}

fn devices_snapshot() -> Vec<Value> {
    let favorites: HashSet<String> = LocalConfig::get_fav().into_iter().collect();
    let mut devices = HashMap::<String, Value>::new();

    for (id, modified, peer) in PeerConfig::peers(None) {
        let fallback_name = nonempty_or(Some(peer.info.hostname.as_str()), id.as_str());
        let name = nonempty_or(
            peer.options.get("alias").map(String::as_str),
            &fallback_name,
        );
        let platform = nonempty_or(Some(peer.info.platform.as_str()), "Unknown");
        let tag = peer
            .options
            .get("corevia-tag")
            .or_else(|| peer.options.get("tag"))
            .cloned()
            .unwrap_or_default();
        let last_seen = peer_last_connected_at(&peer, modified);
        devices.insert(
            id.clone(),
            json!({
                "id": id,
                "name": name,
                "platform": platform,
                "address": id,
                "tag": tag,
                "status": "offline",
                "latency": Value::Null,
                "lastSeenAt": last_seen,
                "favorite": favorites.contains(&id),
                "passwordConfigured": !peer.password.is_empty()
            }),
        );
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    for client in clients_snapshot() {
        let id = client
            .get("peer_id")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned();
        if id.is_empty() {
            continue;
        }
        let disconnected = client
            .get("disconnected")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let authorized = client
            .get("authorized")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let active = client.get("state").and_then(Value::as_str) == Some("active");
        if disconnected || !(authorized || active) {
            continue;
        }
        let (peer_name, platform) = clients_peer_metadata(&id);
        let fallback_name = nonempty_or(Some(peer_name.as_str()), id.as_str());
        let name = nonempty_or(
            client.get("name").and_then(Value::as_str),
            &fallback_name,
        );
        let status = "online";
        let entry = devices.entry(id.clone()).or_insert_with(|| {
            json!({
                "id": id,
                "name": name,
                "platform": nonempty_or(Some(platform.as_str()), "Unknown"),
                "address": id,
                "tag": "",
                "status": "offline",
                "latency": Value::Null,
                "lastSeenAt": now,
                "favorite": favorites.contains(&id),
                "passwordConfigured": false
            })
        });
        if let Some(object) = entry.as_object_mut() {
            object.insert("status".to_owned(), json!(status));
            object.insert("lastSeenAt".to_owned(), json!(now));
        }
    }

    let mut result: Vec<_> = devices.into_values().collect();
    result.sort_by(compare_devices_by_recency);
    result
}

fn peer_last_connected_at(peer: &PeerConfig, modified: SystemTime) -> u64 {
    peer.options
        .get(COREVIA_LAST_CONNECTED_AT_OPTION)
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or_else(|| {
            modified
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        })
}

fn legacy_peer_recent_at(peer_id: &str) -> u64 {
    let peer_filter = Some(vec![peer_id.to_owned()]);
    PeerConfig::get_vec_id_modified_time_path(&peer_filter)
        .into_iter()
        .find(|(id, _, _)| id == peer_id)
        .map(|(_, modified, _)| {
            modified
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        })
        .unwrap_or_default()
}

fn compare_devices_by_recency(left: &Value, right: &Value) -> std::cmp::Ordering {
    let left_last_seen = left.get("lastSeenAt").and_then(Value::as_u64).unwrap_or_default();
    let right_last_seen = right.get("lastSeenAt").and_then(Value::as_u64).unwrap_or_default();
    right_last_seen.cmp(&left_last_seen).then_with(|| {
        left.get("name")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .cmp(right.get("name").and_then(Value::as_str).unwrap_or_default())
    })
}

fn compare_session_history_by_end(left: &Value, right: &Value) -> std::cmp::Ordering {
    let left_ended_at = left
        .get("endedAt")
        .and_then(Value::as_i64)
        .unwrap_or_default();
    let right_ended_at = right
        .get("endedAt")
        .and_then(Value::as_i64)
        .unwrap_or_default();
    right_ended_at.cmp(&left_ended_at)
}

pub(crate) fn mark_recent_peer_connection(peer_id: &str) {
    if peer_id.is_empty() {
        return;
    }
    let mut peer = PeerConfig::load(peer_id);
    let connected_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    peer.options.insert(
        COREVIA_LAST_CONNECTED_AT_OPTION.to_owned(),
        connected_at.to_string(),
    );
    peer.store(peer_id);
}

fn nonempty_or(value: Option<&str>, fallback: &str) -> String {
    value.filter(|value| !value.is_empty()).unwrap_or(fallback).to_owned()
}

fn execute_manage_peers(payload: &Value) -> Result<Value, BridgeError> {
    let peer_id = required_string(payload, "peerId").ok_or_else(|| invalid_field("peerId"))?;
    if !crate::common::is_valid_untrusted_peer_id(peer_id) {
        return Err(BridgeError {
            code: "invalid-peer-id".to_owned(),
            message: "The peer identifier is invalid.".to_owned(),
        });
    }
    let action = required_string(payload, "action").ok_or_else(|| invalid_field("action"))?;
    match action {
        "save" => {
            let is_new_peer = !PeerConfig::exists(peer_id);
            let mut peer = PeerConfig::load(peer_id);
            if is_new_peer {
                peer.options
                    .insert(COREVIA_LAST_CONNECTED_AT_OPTION.to_owned(), "0".to_owned());
            } else if !peer.options.contains_key(COREVIA_LAST_CONNECTED_AT_OPTION) {
                peer.options.insert(
                    COREVIA_LAST_CONNECTED_AT_OPTION.to_owned(),
                    legacy_peer_recent_at(peer_id).to_string(),
                );
            }
            let name = payload.get("name").and_then(Value::as_str).unwrap_or_default().trim();
            let platform = payload
                .get("platform")
                .and_then(Value::as_str)
                .filter(|value| !value.trim().is_empty())
                .unwrap_or(std::env::consts::OS);
            if !name.is_empty() {
                peer.info.hostname = name.to_owned();
                peer.options.insert("alias".to_owned(), name.to_owned());
            }
            peer.info.platform = platform.to_owned();
            if let Some(tag) = payload.get("tag").and_then(Value::as_str) {
                if tag.is_empty() {
                    peer.options.remove("corevia-tag");
                } else {
                    peer.options.insert("corevia-tag".to_owned(), tag.to_owned());
                }
            }
            if let Some(password) = payload.get("password").and_then(Value::as_str) {
                peer.password = password.as_bytes().to_vec();
            }
            peer.store(peer_id);
            Ok(executed(true))
        }
        "remove" => {
            PeerConfig::remove(peer_id);
            let mut favorites = LocalConfig::get_fav();
            favorites.retain(|id| id != peer_id);
            LocalConfig::set_fav(favorites);
            Ok(executed(true))
        }
        "favorite" | "unfavorite" => {
            let mut favorites = LocalConfig::get_fav();
            if action == "favorite" {
                if !favorites.iter().any(|id| id == peer_id) {
                    favorites.push(peer_id.to_owned());
                }
            } else {
                favorites.retain(|id| id != peer_id);
            }
            LocalConfig::set_fav(favorites);
            Ok(executed(true))
        }
        _ => Err(BridgeError {
            code: "unknown-peer-action".to_owned(),
            message: "The peer action is not available.".to_owned(),
        }),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum OperationResolution {
    Capability(Capability),
    Denied(DenialReason),
    Unsupported,
    InvalidPayload(&'static str),
}

fn resolve_operation(operation: &str, payload: &Value) -> OperationResolution {
    if let Some(message) = payload_shape_error(operation, payload) {
        return OperationResolution::InvalidPayload(message);
    }

    match operation {
        "request-screen-capture-access" => {
            OperationResolution::Capability(Capability::RequestScreenCaptureAccess)
        }
        "get-status"
        | "read-settings"
        | "read-identity"
        | "read-screen-capture-status"
        | "read-devices"
        | "read-sessions"
        | "read-terminal"
        | "read-file-transfer"
        | "read-transfer-chunk"
        | "read-session-frame" =>
        {
            OperationResolution::Capability(Capability::ReadSettings)
        }
        "read-temporary-password" => OperationResolution::Capability(Capability::ReadLocalSecret),
        "write-customer-setting" => match required_string(payload, "key") {
            None => OperationResolution::InvalidPayload("A customer setting key is required."),
            Some(key) if !is_customer_support_setting(key) => {
                OperationResolution::Denied(DenialReason::CustomerSettingOnly)
            }
            Some(_) => OperationResolution::Capability(Capability::WriteCustomerSetting),
        },
        "write-global-setting" => match required_string(payload, "key") {
            None => OperationResolution::InvalidPayload("A setting key is required."),
            Some(key) if !is_known_global_setting(key) => {
                OperationResolution::Denied(DenialReason::CapabilityNotAvailable)
            }
            Some(_) => OperationResolution::Capability(Capability::WriteGlobalSetting),
        },
        "start-connection" => OperationResolution::Capability(Capability::StartConnection),
        "approve-connection" => OperationResolution::Capability(Capability::ApproveConnection),
        "close-session" => OperationResolution::Capability(Capability::CloseSession),
        "change-connection-permission" => match required_string(payload, "permission") {
            None => OperationResolution::InvalidPayload("A connection permission is required."),
            Some(permission) if !is_known_connection_permission(permission) => {
                OperationResolution::Denied(DenialReason::UnknownConnectionPermission)
            }
            Some(_) => OperationResolution::Capability(Capability::ChangeConnectionPermission),
        },
        "input-control" => OperationResolution::Capability(Capability::InputControl),
        "send-mouse" | "send-key" => OperationResolution::Capability(Capability::InputControl),
        "switch-display"
        | "refresh-video"
        | "set-session-quality"
        | "set-session-scroll" => OperationResolution::Capability(Capability::InputControl),
        "use-camera" => OperationResolution::Capability(Capability::UseCamera),
        "use-file-transfer"
        | "read-remote-folder"
        | "send-file-to-remote"
        | "download-remote-file" => OperationResolution::Capability(Capability::UseFileTransfer),
        "cleanup-file-transfer" => OperationResolution::Capability(Capability::ReadSettings),
        "use-terminal" | "close-terminal" | "send-terminal-input" => {
            OperationResolution::Capability(Capability::UseTerminal)
        }
        "use-port-forward" => OperationResolution::Capability(Capability::UsePortForward),
        "use-printer" => OperationResolution::Capability(Capability::UsePrinter),
        "record-session" => OperationResolution::Capability(Capability::RecordSession),
        "restart-remote-device" => OperationResolution::Capability(Capability::RestartRemoteDevice),
        "elevate-session" => OperationResolution::Capability(Capability::ElevateSession),
        "toggle-privacy-mode" => OperationResolution::Capability(Capability::TogglePrivacyMode),
        "manage-service" => OperationResolution::Capability(Capability::ManageService),
        "manage-peers" => OperationResolution::Capability(Capability::ManagePeers),
        "send-chat" => OperationResolution::Capability(Capability::SendChat),
        "use-voice-call" => OperationResolution::Capability(Capability::UseVoiceCall),
        // Keeping this explicit makes the permanent denial visible to every
        // adapter instead of making a forbidden request look unknown.
        "system-command" => OperationResolution::Capability(Capability::SystemCommand),
        _ => OperationResolution::Unsupported,
    }
}

fn payload_shape_error(operation: &str, payload: &Value) -> Option<&'static str> {
    let (allowed, required): (&[&str], &[&str]) = match operation {
        "get-status"
        | "read-settings"
        | "read-identity"
        | "read-screen-capture-status"
        | "request-screen-capture-access"
        | "read-temporary-password"
        | "read-devices"
        | "read-sessions"
        | "system-command" => (&[], &[]),
        "read-file-transfer" | "cleanup-file-transfer" => {
            (&["connectionId"], &["connectionId"])
        }
        "read-transfer-chunk" => (
            &["connectionId", "offset", "maxBytes"],
            &["connectionId", "offset", "maxBytes"],
        ),
        "read-terminal" | "close-terminal" | "use-terminal" => {
            (&["connectionId"], &["connectionId"])
        }
        "send-terminal-input" => (
            &["connectionId", "input"],
            &["connectionId", "input"],
        ),
        "read-session-frame" => (&["connectionId"], &["connectionId"]),
        "write-customer-setting" | "write-global-setting" => {
            (&["key", "value"], &["key", "value"])
        }
        "start-connection" => (&["remoteId"], &["remoteId"]),
        "approve-connection" | "close-session" => (&["connectionId"], &["connectionId"]),
        "change-connection-permission" => {
            (
                &["connectionId", "permission", "enabled"],
                &["connectionId", "permission", "enabled"],
            )
        }
        "input-control" => (&["connectionId", "enabled"], &["connectionId"]),
        "use-camera"
        | "use-file-transfer"
        | "use-printer"
        | "restart-remote-device"
        | "elevate-session" => (&["connectionId"], &["connectionId"]),
        "read-remote-folder" => (&["connectionId", "path"], &["connectionId", "path"]),
        "download-remote-file" => (
            &["connectionId", "fileName"],
            &["connectionId", "fileName"],
        ),
        "send-file-to-remote" => (
            &["connectionId", "fileName", "remoteDirectory", "contentBase64"],
            &["connectionId", "fileName", "remoteDirectory", "contentBase64"],
        ),
        "use-voice-call" => (&["connectionId", "enabled"], &["connectionId"]),
        "switch-display" => (&["connectionId", "display"], &["connectionId", "display"]),
        "refresh-video" => (&["connectionId"], &["connectionId"]),
        "set-session-quality" => (
            &["connectionId", "quality"],
            &["connectionId", "quality"],
        ),
        "set-session-scroll" => (
            &["connectionId", "style"],
            &["connectionId", "style"],
        ),
        "send-mouse" => (
            &[
                "connectionId",
                "mask",
                "x",
                "y",
                "alt",
                "ctrl",
                "shift",
                "command",
            ],
            &["connectionId", "mask", "x", "y"],
        ),
        "send-key" => (
            &[
                "connectionId",
                "name",
                "down",
                "press",
                "alt",
                "ctrl",
                "shift",
                "command",
            ],
            &["connectionId", "name", "down"],
        ),
        "record-session" => (
            &["connectionId", "enabled"],
            &["connectionId", "enabled"],
        ),
        "use-port-forward" => (
            &["connectionId", "localPort", "remotePort", "remoteHost"],
            &["connectionId", "localPort", "remotePort"],
        ),
        "toggle-privacy-mode" => (&["connectionId", "enabled"], &["connectionId", "enabled"]),
        "manage-service" => (&["action"], &["action"]),
        "manage-peers" => (
            &["peerId", "action", "name", "platform", "tag", "password"],
            &["peerId", "action"],
        ),
        "send-chat" => (&["connectionId", "message"], &["connectionId", "message"]),
        _ => return None,
    };

    if payload
        .as_object()
        .map(|object| object.keys().any(|key| !allowed.contains(&key.as_str())))
        .unwrap_or(true)
    {
        return Some("The operation payload contains an unknown field.");
    }
    if required.iter().any(|field| payload.get(*field).is_none()) {
        return Some("The operation payload is missing a required field.");
    }
    if matches!(operation, "input-control" | "use-voice-call")
        && payload
            .get("enabled")
            .map(|value| value.as_bool().is_none())
            .unwrap_or(false)
    {
        return Some("The enabled field is invalid.");
    }
    if operation == "use-port-forward" {
        if let Some(remote_host) = payload.get("remoteHost") {
            let valid = remote_host
                .as_str()
                .map(|host| {
                    !host.is_empty()
                        && host.len() <= 255
                        && !host.chars().any(|character| {
                            character.is_whitespace() || character.is_control()
                        })
                })
                .unwrap_or(false);
            if !valid {
                return Some("The remote host is invalid.");
            }
        }
    }
    None
}

fn required_string<'a>(payload: &'a Value, key: &str) -> Option<&'a str> {
    payload.get(key).and_then(Value::as_str).filter(|value| !value.is_empty())
}

fn is_valid_request_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_REQUEST_ID_LENGTH
        && !value.chars().any(|character| character.is_control() || character.is_whitespace())
}

fn denied_response(
    request_id: String,
    operation: String,
    surface: Surface,
    reason: DenialReason,
) -> BridgeResponse {
    BridgeResponse {
        request_id,
        operation,
        surface: surface.as_str().to_owned(),
        profile: crate::corevia_acl::ProductProfile::current().as_str().to_owned(),
        status: BridgeStatus::Denied,
        audit_id: None,
        data: None,
        events: None,
        error: Some(BridgeError {
            code: reason.as_str().to_owned(),
            message: "The request was denied by the Corevia Desk ACL.".to_owned(),
        }),
    }
}

fn error_response(
    request_id: String,
    operation: String,
    surface: Surface,
    code: &str,
    message: &str,
) -> BridgeResponse {
    BridgeResponse {
        request_id,
        operation,
        surface: surface.as_str().to_owned(),
        profile: crate::corevia_acl::ProductProfile::current().as_str().to_owned(),
        status: BridgeStatus::Error,
        audit_id: None,
        data: None,
        events: None,
        error: Some(BridgeError {
            code: code.to_owned(),
            message: message.to_owned(),
        }),
    }
}

/// Serve the bounded bridge over newline-delimited JSON for the local MCP
/// adapter. This is deliberately stdio-only: no listening port or generic
/// network endpoint is opened by the desktop binary.
pub fn run_mcp_stdio() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::BufWriter::new(std::io::stdout());
    let approval_key = std::env::var(MCP_APPROVAL_KEY_ENV)
        .ok()
        .and_then(|key| hex::decode(key).ok());
    let mut used_approval_references = HashSet::new();
    for line in stdin.lock().lines() {
        let response = match line {
        Ok(line) if !line.trim().is_empty() => match serde_json::from_str::<BridgeRequest>(&line) {
                Ok(request) => dispatch_for_surface_with_approval_context(
                    Surface::Mcp,
                    request,
                    approval_key.as_deref(),
                    &mut used_approval_references,
                ),
                Err(error) => BridgeResponse {
                    request_id: "mcp-parse-error".to_owned(),
                    operation: "unknown".to_owned(),
                    surface: Surface::Mcp.as_str().to_owned(),
                    profile: crate::corevia_acl::ProductProfile::current().as_str().to_owned(),
                    status: BridgeStatus::Error,
                    audit_id: None,
                    data: None,
                    events: None,
                    error: Some(BridgeError {
                        code: "invalid-bridge-message".to_owned(),
                        message: format!("Invalid bridge request: {}", error),
                    }),
                },
            },
            Ok(_) => continue,
            Err(error) => {
                hbb_common::log::warn!("Corevia Desk MCP bridge stdin failed: {}", error);
                break;
            }
        };
        if let Ok(serialized) = serde_json::to_string(&response) {
            let _ = writeln!(stdout, "{}", serialized);
            let _ = stdout.flush();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(operation: &str, payload: Value, approval_reference: Option<&str>) -> BridgeRequest {
        BridgeRequest {
            request_id: "bridge-test-1".to_owned(),
            operation: operation.to_owned(),
            payload,
            approval_reference: approval_reference.map(str::to_owned),
            approval_proof: None,
        }
    }

    fn sign_mcp_approval(request: &BridgeRequest, key: &[u8]) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("valid test key");
        mac.update(request.approval_reference.as_deref().unwrap().as_bytes());
        mac.update(&[0]);
        mac.update(request.operation.as_bytes());
        mac.update(&[0]);
        mac.update(serde_json::to_string(&request.payload).unwrap().as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    #[test]
    fn unknown_operations_do_not_become_generic_commands() {
        let response = dispatch(request("run-anything", json!({}), None));
        assert_eq!(response.status, BridgeStatus::Unsupported);
        assert_eq!(response.error.as_ref().map(|error| error.code.as_str()), Some("operation-not-available"));
    }

    #[test]
    fn terminal_stream_is_local_ui_only_and_has_bounded_inputs() {
        assert_eq!(
            resolve_operation(
                "send-terminal-input",
                &json!({"connectionId": "7", "input": "printf safe"})
            ),
            OperationResolution::Capability(Capability::UseTerminal)
        );
        assert_eq!(
            payload_shape_error(
                "send-terminal-input",
                &json!({"connectionId": "7", "input": "printf safe", "extra": true})
            ),
            Some("The operation payload contains an unknown field.")
        );

        let response = dispatch_for_surface(
            Surface::Mcp,
            request("read-terminal", json!({"connectionId": "7"}), None),
        );
        assert_eq!(response.status, BridgeStatus::Denied);
        assert_eq!(
            response.error.as_ref().map(|error| error.code.as_str()),
            Some("capability-not-available")
        );
    }

    #[test]
    fn system_commands_are_denied_by_the_shared_acl() {
        let response = dispatch(request("system-command", json!({}), Some("nuxt-test")));
        assert_eq!(response.status, BridgeStatus::Denied);
        assert_eq!(response.error.as_ref().map(|error| error.code.as_str()), Some("system-command-forbidden"));
    }

    #[test]
    fn temporary_password_is_local_ui_only_and_identity_stays_secret_free() {
        let identity = dispatch(request("read-identity", json!({}), None));
        let identity_data = identity.data.expect("identity snapshot");
        assert!(identity_data.get("temporaryPassword").is_none());
        assert!(identity_data
            .get("temporaryPasswordAvailable")
            .and_then(Value::as_bool)
            .is_some());

        let ui_response = dispatch(request("read-temporary-password", json!({}), None));
        assert_eq!(ui_response.status, BridgeStatus::Accepted);
        assert!(ui_response
            .data
            .as_ref()
            .and_then(|data| data.get("temporaryPassword"))
            .is_some());

        let mcp_response = dispatch_for_surface(
            Surface::Mcp,
            request("read-temporary-password", json!({}), None),
        );
        assert_eq!(mcp_response.status, BridgeStatus::Denied);
        assert_eq!(
            mcp_response.error.as_ref().map(|error| error.code.as_str()),
            Some("capability-not-available")
        );
    }

    #[test]
    fn screen_capture_permission_is_read_only_and_nuxt_local() {
        let ui_response = dispatch(request("read-screen-capture-status", json!({}), None));
        assert_eq!(ui_response.status, BridgeStatus::Accepted);
        assert!(ui_response
            .data
            .as_ref()
            .and_then(|data| data.get("supported"))
            .and_then(Value::as_bool)
            .is_some());

        let mcp_response = dispatch_for_surface(
            Surface::Mcp,
            request("read-screen-capture-status", json!({}), None),
        );
        assert_eq!(mcp_response.status, BridgeStatus::Denied);
        assert_eq!(
            mcp_response.error.as_ref().map(|error| error.code.as_str()),
            Some("capability-not-available")
        );
    }

    #[test]
    fn screen_capture_permission_request_is_local_and_never_calls_macos_from_mcp() {
        assert_eq!(
            resolve_operation("request-screen-capture-access", &json!({})),
            OperationResolution::Capability(Capability::RequestScreenCaptureAccess)
        );

        let mcp_response = dispatch_for_surface(
            Surface::Mcp,
            request("request-screen-capture-access", json!({}), None),
        );
        assert_eq!(mcp_response.status, BridgeStatus::Denied);
        assert_eq!(
            mcp_response.error.as_ref().map(|error| error.code.as_str()),
            Some("capability-not-available")
        );
    }

    #[test]
    fn full_desk_connection_needs_approval() {
        if crate::corevia_acl::ProductProfile::current()
            != crate::corevia_acl::ProductProfile::FullDesk
        {
            return;
        }

        assert_eq!(
            dispatch(request("start-connection", json!({}), None)).status,
            BridgeStatus::Error
        );
        assert_eq!(
            dispatch(request(
                "start-connection",
                json!({"remoteId": "123"}),
                Some("nuxt-connect-approval"),
            ))
            .status,
            BridgeStatus::Accepted
        );
    }

    #[test]
    fn mcp_cannot_start_sensitive_actions_with_a_caller_supplied_reference() {
        if crate::corevia_acl::ProductProfile::current()
            != crate::corevia_acl::ProductProfile::FullDesk
        {
            return;
        }

        let response = dispatch_for_surface(
            Surface::Mcp,
            request(
                "start-connection",
                json!({"remoteId": "synthetic-peer-id"}),
                Some("made-up-approval"),
            ),
        );

        assert_eq!(response.status, BridgeStatus::Denied);
        assert_eq!(
            response.error.as_ref().map(|error| error.code.as_str()),
            Some("mcp-approval-provenance-unavailable")
        );
    }

    #[test]
    fn mcp_approval_proof_is_bound_to_the_action_and_consumed_once() {
        if crate::corevia_acl::ProductProfile::current()
            != crate::corevia_acl::ProductProfile::FullDesk
        {
            return;
        }

        let key = [7_u8; 32];
        let mut request = request(
            "start-connection",
            json!({"remoteId": "123"}),
            Some("mcp-test-approval"),
        );
        request.approval_proof = Some(sign_mcp_approval(&request, &key));
        let mut used = HashSet::new();
        let accepted = dispatch_for_surface_with_approval_context(
            Surface::Mcp,
            request.clone(),
            Some(&key),
            &mut used,
        );
        assert_eq!(accepted.status, BridgeStatus::Accepted);

        let replay = dispatch_for_surface_with_approval_context(
            Surface::Mcp,
            request.clone(),
            Some(&key),
            &mut used,
        );
        assert_eq!(replay.status, BridgeStatus::Denied);
        assert_eq!(
            replay.error.as_ref().map(|error| error.code.as_str()),
            Some("mcp-approval-provenance-unavailable")
        );

        let mut tampered = request;
        tampered.payload = json!({"remoteId": "456"});
        let tampered = dispatch_for_surface_with_approval_context(
            Surface::Mcp,
            tampered,
            Some(&key),
            &mut HashSet::new(),
        );
        assert_eq!(tampered.status, BridgeStatus::Denied);
        assert_eq!(
            tampered.error.as_ref().map(|error| error.code.as_str()),
            Some("mcp-approval-provenance-unavailable")
        );
    }

    #[test]
    fn explicit_relay_suffix_is_normalized_before_peer_validation() {
        let requested = "123/r";
        let normalized = crate::ui_interface::handle_relay_id(requested);
        assert_eq!(normalized, "123");
        assert_ne!(normalized, requested);
        assert!(crate::common::is_valid_untrusted_peer_id(normalized));
    }

    #[test]
    fn permission_names_are_checked_before_authorization() {
        let response = dispatch(request(
            "change-connection-permission",
            json!({"connectionId": "connection-1", "permission": "shell", "enabled": true}),
            Some("nuxt-permission-approval"),
        ));
        assert_eq!(response.status, BridgeStatus::Denied);
        assert_eq!(response.error.as_ref().map(|error| error.code.as_str()), Some("unknown-connection-permission"));
    }

    #[test]
    fn input_control_accepts_optional_enabled_flag() {
        let enabled_payload = json!({"connectionId": "1", "enabled": false});
        assert_eq!(payload_shape_error("input-control", &enabled_payload), None);
        assert_eq!(
            resolve_operation("input-control", &enabled_payload),
            OperationResolution::Capability(Capability::InputControl)
        );

        let default_payload = json!({"connectionId": "1"});
        assert_eq!(payload_shape_error("input-control", &default_payload), None);
    }

    #[test]
    fn port_forward_accepts_a_bounded_optional_remote_host() {
        let default_host = json!({
            "connectionId": "1",
            "localPort": 22000,
            "remotePort": 22
        });
        assert_eq!(payload_shape_error("use-port-forward", &default_host), None);

        let explicit_host = json!({
            "connectionId": "1",
            "localPort": 22000,
            "remotePort": 22,
            "remoteHost": "localhost"
        });
        assert_eq!(payload_shape_error("use-port-forward", &explicit_host), None);

        for remote_host in [
            json!(""),
            json!("host name"),
            json!("x".repeat(256)),
            json!(22),
        ] {
            let payload = json!({
                "connectionId": "1",
                "localPort": 22000,
                "remotePort": 22,
                "remoteHost": remote_host
            });
            assert_eq!(
                payload_shape_error("use-port-forward", &payload),
                Some("The remote host is invalid.")
            );
        }
    }

    #[test]
    fn voice_call_and_input_control_validate_optional_enabled_flags() {
        for operation in ["input-control", "use-voice-call"] {
            let default_enabled = json!({"connectionId": "1"});
            assert_eq!(payload_shape_error(operation, &default_enabled), None);

            for enabled in [json!(true), json!(false)] {
                let payload = json!({"connectionId": "1", "enabled": enabled});
                assert_eq!(payload_shape_error(operation, &payload), None);
            }

            let malformed = json!({"connectionId": "1", "enabled": "false"});
            assert_eq!(
                payload_shape_error(operation, &malformed),
                Some("The enabled field is invalid.")
            );
        }
    }

    #[test]
    fn malformed_requests_are_rejected_without_echoing_payload() {
        let mut request = request("read-settings", json!({"secret": "never-return"}), None);
        request.request_id = "bad id".to_owned();
        let response = dispatch(request);
        assert_eq!(response.status, BridgeStatus::Error);
        assert_eq!(response.error.as_ref().map(|error| error.code.as_str()), Some("invalid-request-id"));
        assert!(serde_json::to_string(&response).unwrap().find("never-return").is_none());
    }

    #[test]
    fn duplicate_clients_keep_the_live_connection_record() {
        let clients = deduplicate_clients(vec![
            json!({
                "id": 42,
                "peer_id": "298636616",
                "state": "closed",
                "authorized": false,
                "disconnected": true
            }),
            json!({
                "id": 42,
                "peer_id": "298636616",
                "state": "active",
                "authorized": true,
                "disconnected": false
            }),
            json!({"id": 43, "peer_id": "100000001", "state": "pending"}),
        ]);

        assert_eq!(clients.len(), 2);
        assert_eq!(clients[0].get("state").and_then(Value::as_str), Some("active"));
        assert_eq!(clients[0].get("peer_id").and_then(Value::as_str), Some("298636616"));
    }

    #[test]
    fn duplicate_clients_keep_transport_metadata_from_the_live_record() {
        let clients = deduplicate_clients(vec![
            json!({
                "id": 44,
                "peer_id": "relay-peer",
                "state": "closed",
                "authorized": false,
                "disconnected": true,
                "direct": true,
                "secured": false
            }),
            json!({
                "id": 44,
                "peer_id": "relay-peer",
                "state": "active",
                "authorized": true,
                "disconnected": false,
                "direct": false,
                "secured": true
            }),
        ]);

        assert_eq!(clients.len(), 1);
        assert_eq!(clients[0].get("direct").and_then(Value::as_bool), Some(false));
        assert_eq!(clients[0].get("secured").and_then(Value::as_bool), Some(true));
    }

    #[test]
    fn clients_without_ids_are_not_collapsed() {
        let clients = deduplicate_clients(vec![
            json!({"peer_id": "298636616", "state": "pending"}),
            json!({"peer_id": "298636616", "state": "pending"}),
        ]);

        assert_eq!(clients.len(), 2);
    }

    #[test]
    fn session_source_defaults_to_incoming_for_legacy_records() {
        assert!(client_is_incoming(&json!({"id": 1})));
        assert!(client_is_incoming(&json!({"id": 2, "incoming": true})));
        assert!(!client_is_incoming(&json!({"id": 3, "incoming": false})));
    }

    #[test]
    fn session_permissions_preserve_view_and_cursor_flags() {
        let permissions = session_permissions(&json!({
            "view_only": true,
            "show_remote_cursor": true
        }));

        assert_eq!(permissions.get("view_only").and_then(Value::as_bool), Some(true));
        assert_eq!(permissions.get("show_remote_cursor").and_then(Value::as_bool), Some(true));
    }

    #[test]
    fn recent_peer_marker_overrides_file_modified_time() {
        let mut peer = PeerConfig::default();
        peer.options
            .insert(COREVIA_LAST_CONNECTED_AT_OPTION.to_owned(), "42".to_owned());
        assert_eq!(peer_last_connected_at(&peer, UNIX_EPOCH), 42);
    }

    #[test]
    fn never_connected_peer_marker_stays_out_of_recent_connections() {
        let mut peer = PeerConfig::default();
        peer.options
            .insert(COREVIA_LAST_CONNECTED_AT_OPTION.to_owned(), "0".to_owned());
        assert_eq!(
            peer_last_connected_at(&peer, UNIX_EPOCH + std::time::Duration::from_secs(42)),
            0
        );
    }

    #[test]
    fn legacy_peer_uses_its_existing_modified_time() {
        let peer = PeerConfig::default();
        assert_eq!(
            peer_last_connected_at(&peer, UNIX_EPOCH + std::time::Duration::from_secs(42)),
            42
        );
    }

    #[test]
    fn recent_devices_sort_newest_first_and_keep_stable_name_order_for_ties() {
        let mut devices = vec![
            json!({"name": "Beta", "lastSeenAt": 42}),
            json!({"name": "Newest", "lastSeenAt": 99}),
            json!({"name": "Alpha", "lastSeenAt": 42}),
        ];
        devices.sort_by(compare_devices_by_recency);
        let names: Vec<_> = devices
            .iter()
            .filter_map(|device| device.get("name").and_then(Value::as_str))
            .collect();
        assert_eq!(names, vec!["Newest", "Alpha", "Beta"]);
    }

    #[test]
    fn session_history_sorts_newest_end_time_first() {
        let mut history = vec![
            json!({"endedAt": 42}),
            json!({"endedAt": 99}),
            json!({"endedAt": null}),
        ];
        history.sort_by(compare_session_history_by_end);
        let ended_at: Vec<_> = history
            .iter()
            .map(|session| session.get("endedAt").and_then(Value::as_i64))
            .collect();
        assert_eq!(ended_at, vec![Some(99), Some(42), None]);
    }
}
