use wasm_bindgen::prelude::*;

mod image;
mod convolve;
mod transform;
mod fractal;
mod linear;
mod probability;
mod synthesis;

pub use image::*;
pub use convolve::*;
pub use transform::*;
pub use fractal::*;
pub use linear::*;
pub use probability::*;
pub use synthesis::*;

#[wasm_bindgen]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}