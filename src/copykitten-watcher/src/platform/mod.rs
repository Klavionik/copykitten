#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
pub mod x11;
#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
pub use x11::X11;

#[cfg(windows)]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;
