use wasm_bindgen::prelude::*;

/// 暗角 & 复古胶片特效
///
/// 对每个像素，根据**到图片中心的距离**调暗四周。
/// 支持 2 种模式：
///
/// | `kind`     | 效果                          | 额外处理        |
/// | ---------- | ----------------------------- | ---------------- |
/// | `vignette` | 纯暗角（四周变暗）            | 无              |
/// | `vintage`  | 暗角 + 暖色调 + 颗粒（胶片感） | 暖色调 + 随机噪点 |
///
/// `strength` 控制暗角强度（0 = 无效果，1 = 最强）。
///
/// # 参数
///
/// - `pixels`:    RGBA 像素数组
/// - `width`:     图片宽度
/// - `height`:    图片高度
/// - `kind`:      模式名称 `"vignette"` 或 `"vintage"`
/// - `strength`:  暗角强度，范围 0.0 ~ 1.0
///
/// # 返回
///
/// 处理后的 RGBA 像素数组。
#[wasm_bindgen]
pub fn vignette(
    pixels: &[u8],
    width: u32,
    height: u32,
    kind: &str,
    strength: f32,
) -> Vec<u8> {
    let len = (width * height * 4) as usize;
    let mut result = vec![0u8; len];
    let w = width as f32;
    let h = height as f32;
    let cx = w / 2.0;
    let cy = h / 2.0;
    // 中心到对角的距离 = √(cx² + cy²)
    let max_dist = (cx * cx + cy * cy).sqrt();

    for y in 0..height {
        for x in 0..width {
            // 1. 算"距中心比" 0~1（0=正中心，1=对角）
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();
            let t = dist / max_dist;

            // 2. 二次衰减：factor = 1 - strength × t²
            let factor = 1.0 - strength * t * t;

            // 3. 找到当前像素 RGBA
            let (mut r, mut g, mut b) = (
                pixels[((y * width + x) * 4) as usize] as f32,
                pixels[((y * width + x) * 4 + 1) as usize] as f32,
                pixels[((y * width + x) * 4 + 2) as usize] as f32,
            );

            if kind == "vintage" {
                // 暖色调：R 增强、B 减弱
                r *= 1.1;
                g *= 1.0;
                b *= 0.9;
            }

            // 应用暗角 + clamp
            r = (r * factor).clamp(0.0, 255.0);
            g = (g * factor).clamp(0.0, 255.0);
            b = (b * factor).clamp(0.0, 255.0);

            // 复古胶片：加颗粒（确定性伪随机，无需 rand 库）
            if kind == "vintage" {
                let noise = pseudo_rand(x, y, 42) as f32 - 128.0;  // -128 ~ 127
                let grain = noise * 0.15;                          // 缩到 ±19
                r = (r + grain).clamp(0.0, 255.0);
                g = (g + grain).clamp(0.0, 255.0);
                b = (b + grain).clamp(0.0, 255.0);
            }

            let out = ((y * width + x) * 4) as usize;
            result[out]     = r as u8;
            result[out + 1] = g as u8;
            result[out + 2] = b as u8;
            result[out + 3] = pixels[out + 3];
        }
    }

    result
}

/// 确定性伪随机（不依赖 rand 库）
///
/// 给定 (x, y, salt) → 稳定返回 0~255 的伪随机字节
fn pseudo_rand(x: u32, y: u32, salt: u32) -> u8 {
    // 简单 LCG：xorshift 变种
    let mut n = x.wrapping_mul(374761393) ^ y.wrapping_mul(668265263) ^ salt;
    n = (n ^ (n >> 13)).wrapping_mul(1274126177);
    n = n ^ (n >> 16);
    (n & 0xFF) as u8
}
