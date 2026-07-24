fn main() {
    println!("cargo:rerun-if-changed=installer.rc");
    println!("cargo:rerun-if-changed=installer.manifest");
    println!("cargo:rerun-if-changed=icons/decomp.ico");

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "windows" {
        println!("cargo:rustc-link-lib=comctl32");
        println!("cargo:rustc-link-lib=shell32");
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=ole32");

        embed_resource::compile("installer.rc", embed_resource::NONE)
            .manifest_optional()
            .expect("failed to compile Windows installer resources");
    }
}
