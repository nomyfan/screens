#![deny(clippy::all)]

use napi_derive::napi;
use screens::Screen;

#[napi]
pub fn screens() -> Vec<Screen> {
    Screen::screens()
}
