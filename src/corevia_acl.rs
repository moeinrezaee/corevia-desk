use std::{
    fmt,
    sync::atomic::{AtomicU64, Ordering},
};

use hbb_common::log;

static NEXT_AUDIT_ID: AtomicU64 = AtomicU64::new(1);

const CUSTOMER_SUPPORT_SETTINGS: &[&str] = &[
    "lang",
    "theme",
    "enable-confirm-closing-tabs",
    "enable-open-new-connections-in-tabs",
    "view-style",
    "view_style",
    "scroll-style",
    "scroll_style",
    "view-only",
    "view_only",
    "enable-keyboard",
    "enable_keyboard",
    "enable-clipboard",
    "enable_clipboard",
    "enable-audio",
    "enable_audio",
    "enable-block-input",
    "enable_block_input",
    "enable-privacy-mode",
    "enable_privacy_mode",
    "allow-ask-for-note",
    "allow_ask_for_note",
    "show-remote-cursor",
    "show_remote_cursor",
    "follow-remote-cursor",
    "follow_remote_cursor",
    "follow-remote-window",
    "follow_remote_window",
    "lock-after-session-end",
    "lock_after_session_end",
    "collapse_toolbar",
    "collapse-toolbar",
    "hide-toolbar",
    "show-quality-monitor",
    "show_quality_monitor",
    "zoom-cursor",
    "swap-left-right-mouse",
    "i444",
    "allow_swap_key",
    "show-my-cursor",
    "show_my_cursor",
    "keyboard_mode",
    "reverse_mouse_wheel",
    "displays_as_individual_windows",
    "use_all_my_displays_for_the_remote_session",
    "edge_scroll_edge_thickness",
    "image_quality",
    "custom_image_quality",
    "trackpad_speed",
    "custom_fps",
];

const CUSTOMER_SUPPORT_CONNECTION_PERMISSIONS: &[&str] =
    &["keyboard", "block_input", "view_only", "show_remote_cursor"];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Surface {
    Flutter,
    Sciter,
    Nuxt,
    Mcp,
    Internal,
}

impl Surface {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Flutter => "flutter",
            Self::Sciter => "sciter",
            Self::Nuxt => "nuxt",
            Self::Mcp => "mcp",
            Self::Internal => "internal",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "flutter" => Some(Self::Flutter),
            "sciter" => Some(Self::Sciter),
            "nuxt" => Some(Self::Nuxt),
            "mcp" => Some(Self::Mcp),
            "internal" => Some(Self::Internal),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProductProfile {
    CustomerSupport,
    FullDesk,
}

impl ProductProfile {
    pub const fn current() -> Self {
        if cfg!(feature = "corevia-customer-support") {
            Self::CustomerSupport
        } else {
            Self::FullDesk
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CustomerSupport => "customer-support",
            Self::FullDesk => "full-desk",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Capability {
    ReadSettings,
    RequestScreenCaptureAccess,
    ReadLocalSecret,
    WriteCustomerSetting,
    WriteGlobalSetting,
    StartConnection,
    ApproveConnection,
    CloseSession,
    ChangeConnectionPermission,
    InputControl,
    UseCamera,
    UseFileTransfer,
    UseTerminal,
    UsePortForward,
    UsePrinter,
    RecordSession,
    RestartRemoteDevice,
    ElevateSession,
    TogglePrivacyMode,
    ManageService,
    ManagePeers,
    SendChat,
    UseVoiceCall,
    ExternalHttpRequest,
    SystemCommand,
}

impl Capability {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadSettings => "read-settings",
            Self::RequestScreenCaptureAccess => "request-screen-capture-access",
            Self::ReadLocalSecret => "read-local-secret",
            Self::WriteCustomerSetting => "write-customer-setting",
            Self::WriteGlobalSetting => "write-global-setting",
            Self::StartConnection => "start-connection",
            Self::ApproveConnection => "approve-connection",
            Self::CloseSession => "close-session",
            Self::ChangeConnectionPermission => "change-connection-permission",
            Self::InputControl => "input-control",
            Self::UseCamera => "use-camera",
            Self::UseFileTransfer => "use-file-transfer",
            Self::UseTerminal => "use-terminal",
            Self::UsePortForward => "use-port-forward",
            Self::UsePrinter => "use-printer",
            Self::RecordSession => "record-session",
            Self::RestartRemoteDevice => "restart-remote-device",
            Self::ElevateSession => "elevate-session",
            Self::TogglePrivacyMode => "toggle-privacy-mode",
            Self::ManageService => "manage-service",
            Self::ManagePeers => "manage-peers",
            Self::SendChat => "send-chat",
            Self::UseVoiceCall => "use-voice-call",
            Self::ExternalHttpRequest => "external-http-request",
            Self::SystemCommand => "system-command",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DenialReason {
    CapabilityNotAvailable,
    ExplicitApprovalRequired,
    InvalidApprovalReference,
    McpApprovalProvenanceUnavailable,
    CustomerSettingOnly,
    SystemCommandForbidden,
    UnknownConnectionPermission,
    CustomerPermissionOnly,
}

impl DenialReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapabilityNotAvailable => "capability-not-available",
            Self::ExplicitApprovalRequired => "explicit-approval-required",
            Self::InvalidApprovalReference => "invalid-approval-reference",
            Self::McpApprovalProvenanceUnavailable => "mcp-approval-provenance-unavailable",
            Self::CustomerSettingOnly => "customer-setting-only",
            Self::SystemCommandForbidden => "system-command-forbidden",
            Self::UnknownConnectionPermission => "unknown-connection-permission",
            Self::CustomerPermissionOnly => "customer-permission-only",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthorizationRequest<'a> {
    pub surface: Surface,
    pub capability: Capability,
    pub approval_reference: Option<&'a str>,
    pub(crate) approval_provenance_verified: bool,
}

impl<'a> AuthorizationRequest<'a> {
    pub const fn new(surface: Surface, capability: Capability) -> Self {
        Self {
            surface,
            capability,
            approval_reference: None,
            approval_provenance_verified: false,
        }
    }

    pub const fn with_approval(
        surface: Surface,
        capability: Capability,
        approval_reference: &'a str,
    ) -> Self {
        Self {
            surface,
            capability,
            approval_reference: Some(approval_reference),
            approval_provenance_verified: false,
        }
    }

    pub(crate) const fn with_verified_mcp_approval(
        capability: Capability,
        approval_reference: &'a str,
    ) -> Self {
        Self {
            surface: Surface::Mcp,
            capability,
            approval_reference: Some(approval_reference),
            approval_provenance_verified: true,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthorizationReceipt {
    pub audit_id: u64,
    pub profile: ProductProfile,
    pub surface: Surface,
    pub capability: Capability,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthorizationError {
    pub reason: DenialReason,
}

impl AuthorizationError {
    pub const fn new(reason: DenialReason) -> Self {
        Self { reason }
    }
}

impl fmt::Display for AuthorizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.reason.as_str())
    }
}

impl std::error::Error for AuthorizationError {}

pub fn authorize(
    request: AuthorizationRequest<'_>,
) -> Result<AuthorizationReceipt, AuthorizationError> {
    let profile = ProductProfile::current();
    let reason = if request.capability == Capability::SystemCommand {
        Some(DenialReason::SystemCommandForbidden)
    } else if !is_capability_allowed(profile, request.capability) {
        Some(DenialReason::CapabilityNotAvailable)
    } else if request.surface == Surface::Mcp
        && requires_explicit_approval(request.capability)
        && !request.approval_provenance_verified
    {
        Some(DenialReason::McpApprovalProvenanceUnavailable)
    } else if requires_explicit_approval(request.capability) {
        match request.approval_reference {
            None => Some(DenialReason::ExplicitApprovalRequired),
            Some(reference) if !is_valid_approval_reference(reference) => {
                Some(DenialReason::InvalidApprovalReference)
            }
            Some(_) => None,
        }
    } else {
        None
    };

    let audit_id = NEXT_AUDIT_ID.fetch_add(1, Ordering::Relaxed);
    if let Some(reason) = reason {
        audit(
            audit_id,
            profile,
            request.surface,
            request.capability,
            false,
            Some(reason),
        );
        Err(AuthorizationError::new(reason))
    } else {
        audit(
            audit_id,
            profile,
            request.surface,
            request.capability,
            true,
            None,
        );
        Ok(AuthorizationReceipt {
            audit_id,
            profile,
            surface: request.surface,
            capability: request.capability,
        })
    }
}

pub fn authorize_setting(
    surface: Surface,
    key: &str,
    write: bool,
) -> Result<AuthorizationReceipt, AuthorizationError> {
    if ProductProfile::current() == ProductProfile::CustomerSupport
        && !is_customer_support_setting(key)
    {
        let audit_id = NEXT_AUDIT_ID.fetch_add(1, Ordering::Relaxed);
        let reason = DenialReason::CustomerSettingOnly;
        audit(
            audit_id,
            ProductProfile::CustomerSupport,
            surface,
            if write {
                Capability::WriteGlobalSetting
            } else {
                Capability::ReadSettings
            },
            false,
            Some(reason),
        );
        return Err(AuthorizationError::new(reason));
    }

    authorize(AuthorizationRequest::new(
        surface,
        if write {
            if ProductProfile::current() == ProductProfile::CustomerSupport {
                Capability::WriteCustomerSetting
            } else {
                Capability::WriteGlobalSetting
            }
        } else {
            Capability::ReadSettings
        },
    ))
}

pub fn authorize_setting_with_approval(
    surface: Surface,
    key: &str,
    write: bool,
    approval_reference: &str,
) -> Result<AuthorizationReceipt, AuthorizationError> {
    if write && key == "stop-service" {
        return authorize(AuthorizationRequest::with_approval(
            surface,
            Capability::ManageService,
            approval_reference,
        ));
    }
    if write && ProductProfile::current() == ProductProfile::FullDesk {
        return authorize(AuthorizationRequest::with_approval(
            surface,
            Capability::WriteGlobalSetting,
            approval_reference,
        ));
    }
    authorize_setting(surface, key, write)
}

pub fn authorize_local_ui(
    surface: Surface,
    write: bool,
) -> Result<AuthorizationReceipt, AuthorizationError> {
    authorize(AuthorizationRequest::new(
        surface,
        if write {
            Capability::WriteCustomerSetting
        } else {
            Capability::ReadSettings
        },
    ))
}

pub fn authorize_session_option(
    surface: Surface,
    name: &str,
    approval_reference: &str,
) -> Result<AuthorizationReceipt, AuthorizationError> {
    let capability = match name {
        "privacy-mode" => Capability::TogglePrivacyMode,
        "disable-audio"
        | "disable-clipboard"
        | "enable-file-copy-paste"
        | "block-input"
        | "unblock-input"
        | "view-only" => Capability::ChangeConnectionPermission,
        name if is_customer_support_setting(name) => {
            return authorize_local_ui(surface, true);
        }
        _ => return Err(AuthorizationError::new(DenialReason::CapabilityNotAvailable)),
    };

    authorize(AuthorizationRequest::with_approval(
        surface,
        capability,
        approval_reference,
    ))
}

pub fn authorize_connection_permission(
    surface: Surface,
    name: &str,
    approval_reference: &str,
) -> Result<AuthorizationReceipt, AuthorizationError> {
    authorize_connection_permission_inner(surface, name, approval_reference, false)
}

pub(crate) fn authorize_connection_permission_with_verified_mcp_approval(
    name: &str,
    approval_reference: &str,
) -> Result<AuthorizationReceipt, AuthorizationError> {
    authorize_connection_permission_inner(Surface::Mcp, name, approval_reference, true)
}

fn authorize_connection_permission_inner(
    surface: Surface,
    name: &str,
    approval_reference: &str,
    mcp_approval_verified: bool,
) -> Result<AuthorizationReceipt, AuthorizationError> {
    if !is_known_connection_permission(name) {
        return Err(AuthorizationError::new(
            DenialReason::UnknownConnectionPermission,
        ));
    }
    if ProductProfile::current() == ProductProfile::CustomerSupport
        && !is_customer_support_connection_permission(name)
    {
        return Err(AuthorizationError::new(
            DenialReason::CustomerPermissionOnly,
        ));
    }

    let request = if mcp_approval_verified {
        AuthorizationRequest::with_verified_mcp_approval(
            Capability::ChangeConnectionPermission,
            approval_reference,
        )
    } else {
        AuthorizationRequest::with_approval(
            surface,
            Capability::ChangeConnectionPermission,
            approval_reference,
        )
    };
    authorize(request)
}

pub fn is_customer_support_setting(key: &str) -> bool {
    CUSTOMER_SUPPORT_SETTINGS.contains(&key)
}

pub fn is_known_connection_permission(name: &str) -> bool {
    matches!(
        name,
        "keyboard"
            | "clipboard"
            | "audio"
            | "file"
            | "restart"
            | "recording"
            | "block_input"
            | "view_only"
            | "show_remote_cursor"
            | "privacy_mode"
    )
}

pub fn is_customer_support_connection_permission(name: &str) -> bool {
    CUSTOMER_SUPPORT_CONNECTION_PERMISSIONS.contains(&name)
}

fn is_capability_allowed(profile: ProductProfile, capability: Capability) -> bool {
    match profile {
        ProductProfile::FullDesk => capability != Capability::SystemCommand,
        ProductProfile::CustomerSupport => matches!(
            capability,
            Capability::ReadSettings
                | Capability::RequestScreenCaptureAccess
                | Capability::ReadLocalSecret
                | Capability::WriteCustomerSetting
                | Capability::ApproveConnection
                | Capability::CloseSession
                | Capability::ChangeConnectionPermission
        ),
    }
}

fn requires_explicit_approval(capability: Capability) -> bool {
    matches!(
        capability,
        Capability::RequestScreenCaptureAccess
            | Capability::StartConnection
            | Capability::ApproveConnection
            | Capability::CloseSession
            | Capability::ChangeConnectionPermission
            | Capability::InputControl
            | Capability::UseCamera
            | Capability::UseFileTransfer
            | Capability::UseTerminal
            | Capability::UsePortForward
            | Capability::UsePrinter
            | Capability::RecordSession
            | Capability::RestartRemoteDevice
            | Capability::ElevateSession
            | Capability::TogglePrivacyMode
            | Capability::ManageService
            | Capability::UseVoiceCall
            | Capability::ExternalHttpRequest
            | Capability::WriteGlobalSetting
        )
}

fn is_valid_approval_reference(reference: &str) -> bool {
    !reference.is_empty()
        && reference.len() <= 128
        && !reference
            .chars()
            .any(|character| character.is_control() || character.is_whitespace())
}

fn audit(
    audit_id: u64,
    profile: ProductProfile,
    surface: Surface,
    capability: Capability,
    allowed: bool,
    reason: Option<DenialReason>,
) {
    if capability == Capability::ReadSettings && allowed {
        return;
    }
    if capability == Capability::InputControl && allowed {
        return;
    }
    log::info!(
        "corevia_acl audit_id={} decision={} profile={} surface={} capability={} reason={}",
        audit_id,
        if allowed { "allow" } else { "deny" },
        profile.as_str(),
        surface.as_str(),
        capability.as_str(),
        reason.map(DenialReason::as_str).unwrap_or("none")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_round_trip_is_explicit() {
        for surface in [
            Surface::Flutter,
            Surface::Sciter,
            Surface::Nuxt,
            Surface::Mcp,
            Surface::Internal,
        ] {
            assert_eq!(Surface::parse(surface.as_str()), Some(surface));
        }
        assert_eq!(Surface::parse("electron"), None);
    }

    #[test]
    fn approval_references_are_validated_without_being_logged() {
        assert!(is_valid_approval_reference("flutter-connection-approval"));
        assert!(!is_valid_approval_reference(""));
        assert!(!is_valid_approval_reference("user approval"));
        assert!(!is_valid_approval_reference("user\napproval"));
    }

    #[test]
    fn sensitive_external_requests_need_approval() {
        if ProductProfile::current() != ProductProfile::FullDesk {
            return;
        }
        assert_eq!(
            authorize(AuthorizationRequest::new(
                Surface::Flutter,
                Capability::ExternalHttpRequest,
            )),
            Err(AuthorizationError::new(
                DenialReason::ExplicitApprovalRequired
            ))
        );
    }

    #[test]
    fn mcp_caller_text_cannot_claim_human_approval() {
        if ProductProfile::current() != ProductProfile::FullDesk {
            return;
        }

        for capability in [
            Capability::StartConnection,
            Capability::ApproveConnection,
            Capability::CloseSession,
            Capability::InputControl,
            Capability::RestartRemoteDevice,
            Capability::WriteGlobalSetting,
        ] {
            assert_eq!(
                authorize(AuthorizationRequest::with_approval(
                    Surface::Mcp,
                    capability,
                    "caller-supplied-reference",
                )),
                Err(AuthorizationError::new(
                    DenialReason::McpApprovalProvenanceUnavailable
                )),
                "{capability:?} must fail closed on MCP until user approval can be verified"
            );
        }
    }

    #[test]
    fn verified_mcp_approval_is_scoped_to_an_explicit_capability() {
        if ProductProfile::current() != ProductProfile::FullDesk {
            return;
        }

        assert!(authorize(AuthorizationRequest::with_verified_mcp_approval(
            Capability::StartConnection,
            "mcp-approval-test",
        ))
        .is_ok());
        assert_eq!(
            authorize(AuthorizationRequest::with_verified_mcp_approval(
                Capability::SystemCommand,
                "mcp-approval-test",
            )),
            Err(AuthorizationError::new(
                DenialReason::SystemCommandForbidden
            ))
        );
    }

    #[test]
    fn system_commands_are_never_authorized() {
        let result = authorize(AuthorizationRequest::new(
            Surface::Mcp,
            Capability::SystemCommand,
        ));
        assert_eq!(
            result,
            Err(AuthorizationError::new(
                DenialReason::SystemCommandForbidden
            ))
        );
    }

    #[test]
    fn full_desk_requires_approval_for_sensitive_actions() {
        if ProductProfile::current() != ProductProfile::FullDesk {
            return;
        }
        assert_eq!(
            authorize(AuthorizationRequest::new(
                Surface::Flutter,
                Capability::RestartRemoteDevice,
            )),
            Err(AuthorizationError::new(
                DenialReason::ExplicitApprovalRequired
            ))
        );
        assert!(authorize(AuthorizationRequest::with_approval(
            Surface::Flutter,
            Capability::RestartRemoteDevice,
            "flutter-restart-approval",
        ))
        .is_ok());
    }

    #[test]
    fn customer_support_policy_is_restricted_when_selected() {
        if ProductProfile::current() != ProductProfile::CustomerSupport {
            return;
        }
        assert!(authorize_setting(Surface::Flutter, "lang", true).is_ok());
        assert_eq!(
            authorize_setting(Surface::Flutter, "api-server", true),
            Err(AuthorizationError::new(DenialReason::CustomerSettingOnly))
        );
        assert_eq!(
            authorize(AuthorizationRequest::with_approval(
                Surface::Flutter,
                Capability::StartConnection,
                "flutter-connect-approval",
            )),
            Err(AuthorizationError::new(
                DenialReason::CapabilityNotAvailable
            ))
        );
        assert!(authorize(AuthorizationRequest::with_approval(
            Surface::Flutter,
            Capability::ApproveConnection,
            "flutter-incoming-approval",
        ))
        .is_ok());
    }

    #[test]
    fn screen_capture_prompt_is_customer_support_only_and_requires_explicit_intent() {
        let capability = Capability::RequestScreenCaptureAccess;
        assert!(requires_explicit_approval(capability));
        assert!(is_capability_allowed(ProductProfile::CustomerSupport, capability));
        assert!(is_capability_allowed(ProductProfile::FullDesk, capability));

        if ProductProfile::current() == ProductProfile::CustomerSupport {
            assert_eq!(
                authorize(AuthorizationRequest::new(Surface::Nuxt, capability)),
                Err(AuthorizationError::new(
                    DenialReason::ExplicitApprovalRequired
                ))
            );
            assert!(authorize(AuthorizationRequest::with_approval(
                Surface::Nuxt,
                capability,
                "nuxt-customer-screen-capture-request",
            ))
            .is_ok());
        } else {
            assert!(authorize(AuthorizationRequest::with_approval(
                Surface::Nuxt,
                capability,
                "nuxt-customer-screen-capture-request",
            ))
            .is_ok());
        }
    }

    #[test]
    fn connection_permission_names_are_allowlisted() {
        assert!(is_known_connection_permission("keyboard"));
        assert!(is_known_connection_permission("privacy_mode"));
        assert!(!is_known_connection_permission("shell"));
        assert!(is_customer_support_connection_permission("keyboard"));
        assert!(!is_customer_support_connection_permission("file"));
    }

    #[test]
    fn customer_support_connection_permissions_stay_narrow() {
        if ProductProfile::current() != ProductProfile::CustomerSupport {
            return;
        }
        assert!(authorize_connection_permission(
            Surface::Nuxt,
            "keyboard",
            "nuxt-keyboard-approval",
        )
        .is_ok());
        assert_eq!(
            authorize_connection_permission(
                Surface::Nuxt,
                "file",
                "nuxt-file-approval",
            ),
            Err(AuthorizationError::new(
                DenialReason::CustomerPermissionOnly,
            ))
        );
    }

    #[test]
    fn session_options_use_the_same_boundary() {
        assert_eq!(
            authorize_session_option(
                Surface::Flutter,
                "unknown-session-option",
                "flutter-session-option-approval",
            ),
            Err(AuthorizationError::new(
                DenialReason::CapabilityNotAvailable
            ))
        );

        if ProductProfile::current() == ProductProfile::CustomerSupport {
            assert!(authorize_session_option(
                Surface::Flutter,
                "view-only",
                "flutter-session-option-approval",
            )
            .is_ok());
            assert!(authorize_session_option(
                Surface::Flutter,
                "show-remote-cursor",
                "flutter-session-option-approval",
            )
            .is_ok());
            assert!(authorize_session_option(
                Surface::Flutter,
                "privacy-mode",
                "flutter-session-option-approval",
            )
            .is_err());
        }
    }
}
