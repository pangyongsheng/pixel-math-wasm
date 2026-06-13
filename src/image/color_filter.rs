use wasm_bindgen::prelude::*;

/// 应用指定的单色滤镜
///
/// 支持 7 种常见艺术滤镜：
///
/// | 滤镜名           | 效果                       | 实现方式               |
/// | ---------------- | -------------------------- | ---------------------- |
/// | `sepia`          | 复古泛黄                    | 经典 sepia 矩阵         |
/// | `cool`           | 冷色调（蓝绿调）            | 减 R 增 B               |
/// | `warm`           | 暖色调（橙红调）            | 增 R 减 B               |
/// | `red_boost`      | 红通道 ×1.5                | 直接乘                  |
/// | `green_boost`    | 绿通道 ×1.5                | 直接乘                  |
/// | `blue_boost`     | 蓝通道 ×1.5                | 直接乘                  |
/// | `grayscale_tint` | 灰度 + 怀旧黄染色            | BT.709 灰度 + 染色       |
///
/// # 参数
///
/// - `pixels`: RGBA 像素数组
/// - `width`:  图片宽度
/// - `height`: 图片高度
/// - `filter`: 滤镜名称字符串（见上表）
///
/// # 返回
///
/// 应用滤镜后的 RGBA 像素数组。
#[wasm_bindgen]
pub fn color_filter(pixels: &[u8], width: u32, height: u32, filter: &str) -> Vec<u8> {
    let len = (width * height * 4) as usize;
    let mut result = Vec::with_capacity(len);

    for i in (0..len).step_by(4) {
        let r = pixels[i] as f32;
        let g = pixels[i + 1] as f32;
        let b = pixels[i + 2] as f32;

        let (nr, ng, nb): (f32, f32, f32) = match filter {
            // 复古泛黄：经典 sepia 矩阵（来自 Microsoft 标准）
            "sepia" => (
                r * 0.393 + g * 0.769 + b * 0.189,
                r * 0.349 + g * 0.686 + b * 0.168,
                r * 0.272 + g * 0.534 + b * 0.131,
            ),
            // 冷色调：减红增蓝
            "cool" => (r * 0.9, g, b * 1.1),
            // 暖色调：增红减蓝
            "warm" => (r * 1.1, g, b * 0.9),
            // 单通道增强
            "red_boost" => (r * 1.5, g, b),
            "green_boost" => (r, g * 1.5, b),
            "blue_boost" => (r, g, b * 1.5),
            // 灰度 + 怀旧黄染色（先灰度，再叠加暖色 tint）
            "grayscale_tint" => {
                let gray = 0.2126 * r + 0.7152 * g + 0.0722 * b;
                // 给灰度叠加 (1.0, 0.95, 0.8) 的暖色调
                (gray * 1.0, gray * 0.95, gray * 0.8)
            }
            // 未知滤镜：原样返回
            _ => (r, g, b),
        };

        result.push(nr.clamp(0.0, 255.0) as u8);
        result.push(ng.clamp(0.0, 255.0) as u8);
        result.push(nb.clamp(0.0, 255.0) as u8);
        result.push(pixels[i + 3]); // Alpha
    }

    result
}
