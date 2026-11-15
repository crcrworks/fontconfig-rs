fn main() {
    println!("cargo:rerun-if-env-changed=RUST_FONTCONFIG_DLOPEN");
    if std::env::var_os("RUST_FONTCONFIG_DLOPEN").is_some() {
        println!("cargo:rustc-cfg=feature=\"dlopen\"");
    }
    // Fork note: the crate root always declares the fontconfig bindings, so the link dependency
    // is unconditional. `dlopen` used to skip this, which silently removed the root bindings from
    // any graph where another crate had turned the feature on (feature unification). pkg-config
    // supplies non-standard search paths; when it is unavailable (a machine with only the runtime
    // library, which is what `dlopen` was for), a plain `-l` still resolves from the default ones.
    if pkg_config::find_library("fontconfig").is_err() {
        println!("cargo:rustc-link-lib=fontconfig");
    }
}
