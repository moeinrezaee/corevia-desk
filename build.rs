fn generate_version() {
    println!("cargo:rerun-if-env-changed=COREVIA_DESK_VERSION");
    hbb_common::gen_version();

    if let Ok(version) = std::env::var("COREVIA_DESK_VERSION") {
        let generated = std::fs::read_to_string("src/version.rs")
            .expect("the Rust version file should have been generated");
        let build_metadata = generated
            .split_once('\n')
            .map(|(_, rest)| rest)
            .unwrap_or_default();
        let source = format!("pub const VERSION: &str = {version:?};\n{build_metadata}");
        std::fs::write("src/version.rs", source)
            .expect("the release version should be written to the Rust version file");
    }
}

fn build_windows() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os != "windows" {
        return;
    }

    let file = "src/platform/windows.cc";
    let file2 = "src/platform/windows_delete_test_cert.cc";
    let mut build = cc::Build::new();
    println!("cargo:rerun-if-env-changed=COREVIA_CROSS_AR");
    println!("cargo:rerun-if-env-changed=COREVIA_CROSS_RANLIB");
    if let Ok(archiver) = std::env::var("COREVIA_CROSS_AR") {
        build.archiver(archiver);
    }
    if let Ok(ranlib) = std::env::var("COREVIA_CROSS_RANLIB") {
        build.ranlib(ranlib);
    }
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() != Ok("msvc") {
        build.flag("-Wno-c++11-narrowing");
    }
    build.file(file).file(file2).compile("windows");
    println!("cargo:rustc-link-lib=WtsApi32");
    println!("cargo:rerun-if-changed={}", file);
    println!("cargo:rerun-if-changed={}", file2);
}

#[cfg(target_os = "macos")]
fn build_mac() {
    let file = "src/platform/macos.mm";
    let mut b = cc::Build::new();
    if let Ok(os_version::OsVersion::MacOS(v)) = os_version::detect() {
        let v = v.version;
        if v.contains("10.14") {
            b.flag("-DNO_InputMonitoringAuthStatus=1");
        }
    }
    b.flag("-std=c++17").file(file).compile("macos");
    println!("cargo:rerun-if-changed={}", file);
}

#[cfg(windows)]
fn build_manifest() {
    use std::io::Write;
    let mut res = winres::WindowsResource::new();
    res.set_icon("res/icon.ico")
        .set_language(winapi::um::winnt::MAKELANGID(
            winapi::um::winnt::LANG_ENGLISH,
            winapi::um::winnt::SUBLANG_ENGLISH_US,
        ))
        .set_manifest_file("res/manifest.xml");
    match res.compile() {
        Err(e) => {
            write!(std::io::stderr(), "{}", e).unwrap();
            std::process::exit(1);
        }
        Ok(_) => {}
    }
}

fn install_android_deps() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os != "android" {
        return;
    }
    let mut target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if target_arch == "x86_64" {
        target_arch = "x64".to_owned();
    } else if target_arch == "x86" {
        target_arch = "x86".to_owned();
    } else if target_arch == "aarch64" {
        target_arch = "arm64".to_owned();
    } else {
        target_arch = "arm".to_owned();
    }
    let target = format!("{}-android", target_arch);
    let vcpkg_root = std::env::var("VCPKG_ROOT").unwrap();
    let mut path: std::path::PathBuf = vcpkg_root.into();
    if let Ok(vcpkg_root) = std::env::var("VCPKG_INSTALLED_ROOT") {
        path = vcpkg_root.into();
    } else {
        path.push("installed");
    }
    path.push(target);
    println!(
        "cargo:rustc-link-search={}",
        path.join("lib").to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=ndk_compat");
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=OpenSLES");
}

fn main() {
    generate_version();
    install_android_deps();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "windows" {
        build_windows();
        #[cfg(windows)]
        build_manifest();
    }
    if target_os == "macos" {
        #[cfg(target_os = "macos")]
        build_mac();
        println!("cargo:rustc-link-lib=framework=ApplicationServices");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
