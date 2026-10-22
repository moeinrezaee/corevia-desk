//! Product identity for the Corevia Desk distribution.
//!
//! Protocol constants and native ABI symbols that still contain the upstream
//! RustDesk name are intentionally kept unchanged for interoperability.

// Release profiles must not share the native configuration/IPC namespace.
// The macOS bundle identifier alone is not enough: the Rust config layer also
// derives its files, sockets, logs, and service names from APP_NAME.
#[cfg(feature = "corevia-customer-support")]
pub const APP_NAME: &str = "Corevia Desk Customer Support";
#[cfg(not(feature = "corevia-customer-support"))]
pub const APP_NAME: &str = "Corevia Desk";
// Keep the visible app title independent from APP_NAME: APP_NAME also owns the
// persisted configuration namespace, which must stay stable across upgrades.
#[cfg(feature = "corevia-customer-support")]
pub const APP_DISPLAY_NAME: &str = "Corevia Desk - CustomerSupport";
#[cfg(not(feature = "corevia-customer-support"))]
pub const APP_DISPLAY_NAME: &str = "Corevia Desk";
pub const APP_BUNDLE_NAME: &str = "CoreviaDesk";
pub const APP_URI_SCHEME: &str = "corevia-desk";
#[cfg(feature = "corevia-customer-support")]
pub const APP_BUNDLE_IDENTIFIER: &str = "com.corevia.desk.customer-support";
#[cfg(not(feature = "corevia-customer-support"))]
pub const APP_BUNDLE_IDENTIFIER: &str = "com.corevia.desk";
#[cfg(feature = "corevia-customer-support")]
pub const APP_ORGANIZATION: &str = "com.corevia.customer-support";
#[cfg(not(feature = "corevia-customer-support"))]
pub const APP_ORGANIZATION: &str = "com.corevia";

// These are the initial Corevia network defaults. They are installed as
// defaults only: an explicit local setting or a packaged client setting wins.
// Keep this small boundary replaceable when signed wallet-backed updates are
// introduced later.
pub const DEFAULT_ID_SERVER: &str = "87.107.8.155";
pub const DEFAULT_RELAY_SERVER: &str = "87.107.8.155";
pub const DEFAULT_SERVER_KEY: &str = "XEccEs7Fif1tG7WsdQl7NCc3sU0AwTVLjFo9WKIzbXM=";

pub fn app_bundle_name() -> String {
    let app_name = hbb_common::config::APP_NAME
        .read()
        .map(|name| name.clone())
        .unwrap_or_else(|_| APP_NAME.to_owned());
    if app_name == APP_NAME {
        APP_BUNDLE_NAME.to_owned()
    } else {
        app_name
            .chars()
            .filter(|character| character.is_ascii_alphanumeric())
            .collect()
    }
}

pub fn initialize() {
    if let Ok(mut app_name) = hbb_common::config::APP_NAME.write() {
        if *app_name == "RustDesk" {
            *app_name = APP_NAME.to_owned();
        }
    } else {
        hbb_common::log::error!("Failed to initialize Corevia Desk app name");
    }

    // Keep the network configuration in the shared Rust config layer so the
    // existing Flutter client and future web shells use the same defaults.
    // `entry` is intentional: custom-client defaults loaded later must remain
    // authoritative, and a user's saved value is handled by Config::get_option.
    if let Ok(mut defaults) = hbb_common::config::DEFAULT_SETTINGS.write() {
        defaults
            .entry(hbb_common::config::keys::OPTION_CUSTOM_RENDEZVOUS_SERVER.to_owned())
            .or_insert_with(|| DEFAULT_ID_SERVER.to_owned());
        defaults
            .entry(hbb_common::config::keys::OPTION_RELAY_SERVER.to_owned())
            .or_insert_with(|| DEFAULT_RELAY_SERVER.to_owned());
        defaults
            .entry(hbb_common::config::keys::OPTION_KEY.to_owned())
            .or_insert_with(|| DEFAULT_SERVER_KEY.to_owned());
    } else {
        hbb_common::log::error!("Failed to initialize Corevia Desk network defaults");
    }

    // Container deployments can provide a private server without rebuilding
    // the binary. Environment values are deliberately read by Rust here, at
    // the same boundary as the built-in defaults, and are never written to
    // logs. The desktop UI can still change these values through the approved
    // Full Desk settings flow.
    for (environment, key) in [
        ("COREVIA_ID_SERVER", hbb_common::config::keys::OPTION_CUSTOM_RENDEZVOUS_SERVER),
        ("COREVIA_RELAY_SERVER", hbb_common::config::keys::OPTION_RELAY_SERVER),
        ("COREVIA_API_SERVER", hbb_common::config::keys::OPTION_API_SERVER),
        ("COREVIA_SERVER_KEY", hbb_common::config::keys::OPTION_KEY),
    ] {
        if let Some(value) = environment_value(environment) {
            hbb_common::config::Config::set_option(key.to_owned(), value);
        }
    }

    // COREVIA_DESK_PORT is the optional direct-access port for a container or
    // a controlled Linux host. The ID/Relay ports remain configurable by
    // including :port in COREVIA_ID_SERVER/COREVIA_RELAY_SERVER.
    if let Some(port) = environment_port("COREVIA_DESK_PORT") {
        hbb_common::config::Config::set_option(
            hbb_common::config::keys::OPTION_DIRECT_SERVER.to_owned(),
            "Y".to_owned(),
        );
        hbb_common::config::Config::set_option(
            hbb_common::config::keys::OPTION_DIRECT_ACCESS_PORT.to_owned(),
            port.to_string(),
        );
    }

    #[cfg(target_os = "macos")]
    if let Ok(mut organization) = hbb_common::config::ORG.write() {
        *organization = APP_ORGANIZATION.to_owned();
    } else {
        hbb_common::log::error!("Failed to initialize Corevia Desk organization");
    }
}

fn environment_value(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn environment_port(name: &str) -> Option<u16> {
    environment_value(name).and_then(|value| value.parse::<u16>().ok()).filter(|port| *port > 0)
}
