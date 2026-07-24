fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "windows" {
        embed_resource::compile("desktop.rc", embed_resource::NONE)
            .manifest_optional()
            .expect("failed to compile cozip_desktop resources");

        println!("cargo:rustc-link-lib=comctl32");
        println!("cargo:rustc-link-lib=powrprof");
        println!("cargo:rustc-link-lib=shell32");

        let mut build = cc::Build::new();
        build
            .cpp(true)
            .std("c++14")
            .file("../cozip/src/unrar_win_fix.cpp");
        build.compile("unrar_win_fix_desktop");

        println!("cargo:rustc-link-arg=-Wl,--undefined=_Z5WinNTv");
        println!("cargo:rustc-link-arg=-Wl,--undefined=_Z20IsWindows11OrGreaterv");
    }
}
