#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[derive(Debug, Clone, Copy)]
pub struct Screen {
    pub logical_size: (f64, f64),
    #[cfg(target_os = "macos")]
    /// Logical visible size.
    pub visible_size: (f64, f64),
    /// Physical resolution.
    pub resolution: (u32, u32),
    /// Whether the screen is the primary screen.
    pub primary: bool,
}
