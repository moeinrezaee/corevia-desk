use cc::Build;

fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-env-changed=COREVIA_CROSS_AR");
    println!("cargo:rerun-if-env-changed=COREVIA_CROSS_RANLIB");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        println!("cargo:rustc-link-lib=Kernel32");
        let mut build = Build::new();
        if let Ok(archiver) = std::env::var("COREVIA_CROSS_AR") {
            build.archiver(archiver);
        }
        if let Ok(ranlib) = std::env::var("COREVIA_CROSS_RANLIB") {
            build.ranlib(ranlib);
        }
        build.file("src/win.cpp").compile("machine-uid");
    }
}
