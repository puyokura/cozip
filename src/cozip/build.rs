fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "windows" {
        println!("cargo:rustc-link-lib=powrprof");
        println!("cargo:rustc-link-lib=shell32");

        let mut build = cc::Build::new();
        build
            .cpp(true)
            .std("c++14")
            .file("src/unrar_win_fix.cpp");
        build.compile("unrar_win_fix");
    }
}
