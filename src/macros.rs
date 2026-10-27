/// Includes the code that [tighterror-build] writes for a module in a
/// build script.
///
/// The argument is the name of the module in the specification file. The
/// macro expands to the items of the module, so it is called inside the
/// `mod` item that holds them:
///
/// ```text
/// /// Crate errors.
/// pub mod errors {
///     tighterror::include_errors!(errors);
/// }
/// ```
///
/// The crate's build script must generate the code with `BuildOptions` of
/// [tighterror-build].
///
/// [tighterror-build]: https://docs.rs/tighterror-build/latest/tighterror_build
#[macro_export]
macro_rules! include_errors {
    ($module:ident) => {
        ::core::include!(::core::concat!(
            ::core::env!("OUT_DIR"),
            "/",
            ::core::stringify!($module),
            ".rs"
        ));
    };
}
