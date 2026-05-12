use wasm_bindgen::prelude::*;

mod image;
mod transform;
mod fractal;
mod linear;
mod probability;

pub use image::*;
pub use transform::*;
pub use fractal::*;
pub use linear::*;
pub use probability::*;

#[wasm_bindgen]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}