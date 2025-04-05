#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(feature = "napi")]
use napi_derive::napi;

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Debug, Clone, Copy)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl From<(u32, u32)> for Resolution {
    fn from(value: (u32, u32)) -> Self {
        Self {
            width: value.0,
            height: value.1,
        }
    }
}

#[cfg_attr(feature = "napi", napi(object))]
#[derive(Debug, Clone, Copy)]
pub struct Screen {
    /// The dimensions and location of the screen.
    pub frame: Rect,
    /// The current location and dimensions of the visible screen.
    pub visible_frame: Rect,
    /// Native resolution.
    pub resolution: Resolution,
    /// Whether the screen is the primary screen.
    pub primary: bool,
}
