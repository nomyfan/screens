#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Screen {
    pub position: Rect,
    #[cfg(target_os = "macos")]
    /// Logical visible size.
    pub visible_position: Rect,
    /// Native resolution.
    pub resolution: (u32, u32),
    /// Whether the screen is the primary screen.
    pub primary: bool,
}
