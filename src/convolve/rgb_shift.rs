use wasm_bindgen::prelude::*;

/// 故障风 RGB 通道偏移
///
/// 把 R、G、B 三通道**分别偏移**不同位置，制造"赛博朋克 / 故障"效果。
///
/// 支持 2 种模式：
///
/// | `kind`       | 效果                                          |
/// | ------------ | --------------------------------------------- |
/// | `horizontal` | 整张图**统一偏移**：R 左移、B 右移、G 不动   |
/// | `random`     | **每行随机偏移**：更像"真故障"               |
///
/// # 参数
///
/// - `pixels`: RGBA 像素数组
/// - `width`:  图片宽度
/// - `height`: 图片高度
/// - `kind`:   模式 `"horizontal"` 或 `"random"`
/// - `offset`: 偏移像素数（0~30 常用）
///
/// # 返回
///
/// 故障处理后的 RGBA 像素数组。
#[wasm_bindgen]
pub fn rgb_shift(
    pixels: &[u8],
    width: u32,
    height: u32,
    kind: &str,
    offset: u32,
) -> Vec<u8> {
    let len = (width * height * 4) as usize;
    let mut result = vec![0u8; len];
    let w = width as i32;
    let h = height as i32;
    let off = offset as i32;

    for y in 0..h {
        // 每行用一个偏移（uniform 模式：全部相同；random 模式：每行不同）
        let row_off = if kind == "random" {
            // 用 xorshift 风格伪随机（不需要 rand 库）
            (pseudo_rand(y as u32, 0, 42) as i32 % (off * 2 + 1)) - off
        } else {
            off
        };

        for x in 0..w {
            let out = ((y * w + x) * 4) as usize;

            // R 通道：向左偏移 row_off 像素（clamp 边界）
            let r_x = (x - row_off).clamp(0, w - 1);
            let r_idx = ((y * w + r_x) * 4) as usize;
            result[out] = pixels[r_idx];

            // G 通道：保持原位
            result[out + 1] = pixels[out + 1];

            // B 通道：向右偏移 row_off 像素
            let b_x = (x + row_off).clamp(0, w - 1);
            let b_idx = ((y * w + b_x) * 4) as usize;
            result[out + 2] = pixels[b_idx + 2];

            // A 通道：保持原位
            result[out + 3] = pixels[out + 3];
        }
    }

    result
}

/// 确定性伪随机（与 vignette.rs 同款）
fn pseudo_rand(x: u32, y: u32, salt: u32) -> u8 {
    let mut n = x.wrapping_mul(374761393) ^ y.wrapping_mul(668265263) ^ salt;
    n = (n ^ (n >> 13)).wrapping_mul(1274126177);
    n = n ^ (n >> 16);
    (n & 0xFF) as u8
}
