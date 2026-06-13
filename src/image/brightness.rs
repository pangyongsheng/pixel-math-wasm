use wasm_bindgen::prelude::*;

/// 亮度 / 对比度调节
///
/// 对每个 RGB 像素：
/// 1. 先调对比度（围绕 128 拉伸 / 压缩）
/// 2. 再调亮度（直接相加）
/// 3. 截断到 0~255
///
/// # 参数
///
/// - `pixels`:     RGBA 像素数组
/// - `width`:      图片宽度
/// - `height`:     图片高度
/// - `brightness`: 亮度偏移，范围 -100 ~ +100
/// - `contrast`:   对比度，范围 -100 ~ +100
///                  - 0 表示不变
///                  - 正数增强对比度（最大 +100，相当于 ×2）
///                  - 负数降低对比度（最小 -100，变成纯灰）
///
/// # 返回
///
/// 调整后的 RGBA 像素数组。
#[wasm_bindgen]
pub fn brightness_contrast(
    pixels: &[u8],
    width: u32,
    height: u32,
    brightness: i32,
    contrast: i32,
) -> Vec<u8> {
    let len = (width * height * 4) as usize;
    let mut result = Vec::with_capacity(len);

    // 预计算对比度因子：contrast 范围 -100~100 → factor 范围 0~2
    // 公式：factor = 1 + contrast / 100
    //   contrast = 0    → factor = 1 （不变）
    //   contrast = 100  → factor = 2 （×2）
    //   contrast = -100 → factor = 0 （全变 128）
    let factor: f32 = 1.0 + (contrast as f32 / 100.0);
    let b = brightness as f32;

    for i in (0..len).step_by(4) {
        // RGB 三通道独立处理
        for c in 0..3 {
            let old = pixels[i + c] as f32;
            // 1) 调对比度：以 128 为中心拉伸
            let contrasted = (old - 128.0) * factor + 128.0;
            // 2) 调亮度：直接相加
            let adjusted = contrasted + b;
            // 3) 截断到 0~255
            result.push(adjusted.clamp(0.0, 255.0) as u8);
        }
        // Alpha 通道保持不变
        result.push(pixels[i + 3]);
    }

    result
}
