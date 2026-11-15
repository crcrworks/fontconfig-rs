// Cargo unifies features across a dependency graph. Turning `dlopen` on for one consumer used to
// delete the root bindings that another consumer imports directly (servo-fonts imports
// `FcConfigGetCurrent` and friends from this crate's root). Taking each function's address needs
// the name to exist at compile time and the symbol to resolve at link time, so this is the minimal
// check that both hold with the feature off and on.
#[test]
fn the_root_bindings_exist() {
    let _: unsafe extern "C" fn() -> *mut fontconfig_sys::FcConfig =
        fontconfig_sys::FcConfigGetCurrent;
    let _: unsafe extern "C" fn(
        *mut fontconfig_sys::FcConfig,
        *mut fontconfig_sys::FcPattern,
        fontconfig_sys::FcMatchKind,
    ) -> fontconfig_sys::FcBool = fontconfig_sys::FcConfigSubstitute;
}

// The handle fontique uses is still there when the feature is on. Naming it is the check: `LIB`
// opens the library by soname, so reading it would assert about this machine's search path.
#[cfg(feature = "dlopen")]
#[test]
fn the_dlopen_handle_is_still_declared() {
    fn handle() -> &'static fontconfig_sys::Fc {
        &fontconfig_sys::statics::LIB
    }
    let _ = handle as fn() -> &'static fontconfig_sys::Fc;
}
