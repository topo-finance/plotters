#[cfg(feature = "svg")]
mod svg;
#[cfg(feature = "svg")]
pub use self::svg::SVGBackend;

#[cfg(feature = "bitmap")]
mod bitmap;
#[cfg(feature = "bitmap")]
pub use bitmap::BitMapBackend;

#[cfg(feature = "bitmap")]
pub mod bitmap_pixel {
    pub use super::bitmap::{BGRXPixel, PixelFormat, RGBPixel};
}

#[cfg(target_arch = "wasm32")]
mod canvas;
#[cfg(target_arch = "wasm32")]
pub use canvas::CanvasBackend;

#[cfg(test)]
mod mocked;
#[cfg(test)]
pub use mocked::{create_mocked_drawing_area, MockedBackend};

#[cfg(all(not(target_arch = "wasm32"), feature = "piston"))]
mod piston;
#[cfg(all(not(target_arch = "wasm32"), feature = "piston"))]
pub use piston::{draw_piston_window, PistonBackend};

#[cfg(all(not(target_arch = "wasm32"), feature = "iced_back"))]
mod iced_back;
#[cfg(all(not(target_arch = "wasm32"), feature = "iced_back"))]
pub use iced_back::IcedCanvasBackend;

#[cfg(all(not(target_arch = "wasm32"), feature = "cairo-rs"))]
mod cairo;
#[cfg(all(not(target_arch = "wasm32"), feature = "cairo-rs"))]
pub use self::cairo::CairoBackend;

/// This is the dummy backend placeholder for the backend that never fails
#[derive(Debug)]
pub struct DummyBackendError;

impl std::fmt::Display for DummyBackendError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for DummyBackendError {}
