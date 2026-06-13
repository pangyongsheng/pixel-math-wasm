use wasm_bindgen::prelude::*;

/// 图片反色 / 负片效果
///
/// 把每个 RGB 通道的值用 `255 - 原值` 反转，生成底片风格。
///
/// 公式：`new_channel = 255 - old_channel`
///
/// Alpha 通道保持不变。
///
/// # 参数
///
/// - `pixels`: RGBA 像素数组，每 4 字节代表一个像素 `[R, G, B, A, R, G, B, A, ...]`
/// - `width`:  图片宽度（像素）
/// - `height`: 图片高度（像素）
///
/// # 返回
///
/// 反色后的 RGBA 像素数组。
#[wasm_bindgen]
pub fn invert(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let len = (width * height * 4) as usize;
    let mut result = Vec::with_capacity(len);
    for i in (0..len).step_by(4) {
        // RGB 三通道各自取反，Alpha 保留
        result.push(255 - pixels[i]);     // R
        result.push(255 - pixels[i + 1]); // G
        result.push(255 - pixels[i + 2]); // B
        result.push(pixels[i + 3]);       // A
    }
    result
}
