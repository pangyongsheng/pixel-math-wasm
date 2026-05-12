use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn grayscale(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let len = (width * height * 4) as usize;
    let mut result = Vec::with_capacity(len);

    for i in (0..len).step_by(4) {
        let r = pixels[i] as f32;
        let g = pixels[i + 1] as f32;
        let b = pixels[i + 2] as f32;

        // 加权灰度算法
        let gray = (0.299 * r + 0.587 * g + 0.114 * b) as u8;

        result.push(gray);
        result.push(gray);
        result.push(gray);
        result.push(pixels[i + 3]); // 保留 Alpha 通道
    }

    result
}