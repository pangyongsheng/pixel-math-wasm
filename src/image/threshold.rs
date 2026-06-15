use wasm_bindgen::prelude::*;

/// 手动阈值二值化
///
/// 1. 先用 BT.709 灰度公式把图片转成灰度
/// 2. 每个像素：`gray > threshold ? 255 : 0`
/// 3. 输出黑白图（RGB 三通道都一样）
///
/// # 参数
///
/// - `pixels`:    RGBA 像素数组
/// - `width`:     图片宽度
/// - `height`:    图片高度
/// - `threshold`: 阈值 0~255
///                  - `gray > threshold` → 白 (255)
///                  - `gray <= threshold` → 黑 (0)
///
/// # 返回
///
/// 二值化后的 RGBA 像素数组。
#[wasm_bindgen]
pub fn threshold_with(
    pixels: &[u8],
    width: u32,
    height: u32,
    threshold: u8,
) -> Vec<u8> {
    let len = (width * height * 4) as usize;
    let mut result = Vec::with_capacity(len);

    for i in (0..len).step_by(4) {
        // BT.709 灰度
        let gray = (0.2126 * pixels[i] as f32
                  + 0.7152 * pixels[i + 1] as f32
                  + 0.0722 * pixels[i + 2] as f32) as u8;

        // 二值化
        let bin: u8 = if gray > threshold { 255 } else { 0 };

        result.push(bin);
        result.push(bin);
        result.push(bin);
        result.push(pixels[i + 3]); // Alpha
    }

    result
}

/// Otsu 自动阈值二值化
///
/// 自动算出"最佳阈值"，再按该阈值二值化。
///
/// **Otsu 算法核心**（1979 年提出，至今仍是图像分割经典）：
///
/// 遍历 0~255 的每个候选阈值 t，分成"暗"和"亮"两类，
/// 找出"使类间方差最大"的 t，就是最佳阈值。
///
/// 类间方差 = w0 × w1 × (μ0 - μ1)²
///   - w0 / w1: 两类像素占比
///   - μ0 / μ1: 两类平均灰度
///
/// 直觉：让"暗"和"亮"两堆像素**分得越开越好**。
#[wasm_bindgen]
pub fn threshold_otsu(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let pixel_count = (width * height) as usize;
    let len = pixel_count * 4;

    // 1. 一次遍历：灰度化 + 统计直方图
    let mut histogram = [0u32; 256];

    for i in (0..len).step_by(4) {
        let gray = (0.2126 * pixels[i] as f32
                  + 0.7152 * pixels[i + 1] as f32
                  + 0.0722 * pixels[i + 2] as f32) as u8;
        histogram[gray as usize] += 1;
    }

    // 2. 总像素数 + 总灰度和
    let total: f32 = pixel_count as f32;
    let sum_all: f32 = (0..256)
        .map(|i| (i as f32) * (histogram[i] as f32))
        .sum();

    // 3. 遍历找"类间方差最大"的阈值
    let mut best_threshold: u8 = 0;
    let mut max_variance: f32 = 0.0;

    let mut sum_bg: f32 = 0.0; // 背景（<= t）灰度总和
    let mut weight_bg: f32 = 0.0; // 背景像素数

    for t in 0..256 {
        weight_bg += histogram[t] as f32;
        // 未统计到像素，跳过 不用计算类间方差
        if weight_bg == 0.0 {
            continue;
        }

        let weight_fg = total - weight_bg;
        if weight_fg == 0.0 {
            break;
        }

        sum_bg += (t as f32) * (histogram[t] as f32);

        let mean_bg = sum_bg / weight_bg;
        let mean_fg = (sum_all - sum_bg) / weight_fg;

        // 类间方差
        let variance = weight_bg * weight_fg * (mean_bg - mean_fg).powi(2);

        if variance > max_variance {
            max_variance = variance;
            best_threshold = t as u8;
        }
    }

    // 4. 用最佳阈值二值化（直接调前面的 threshold_with）
    threshold_with(pixels, width, height, best_threshold)
}
