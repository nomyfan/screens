#[cfg(target_os = "macos")]
mod macos;

#[derive(Debug, Clone, Copy)]
pub struct Screen {
    /// Logical width of the screen.
    pub width: f64,
    /// Logical height of the screen.
    pub height: f64,
    /// Logical visible width of the screen.
    #[cfg(target_os = "macos")]
    pub visible_width: f64,
    /// Logical visible height of the screen.
    #[cfg(target_os = "macos")]
    pub visible_height: f64,
    /// Scale factor of the screen.
    pub scale_factor: f64,
    /// Whether the screen is the main screen.
    pub is_main: bool,
}
