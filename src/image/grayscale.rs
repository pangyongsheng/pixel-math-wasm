use wasm_bindgen::prelude::*;

/// 灰度算法枚举
///
/// 在 JS 端可直接传字符串：`"luminance"` / `"luminance709"` / `"average"` / `"max"` / `"min"` / `"desaturate"`
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrayAlgorithm {
    /// 加权平均（ITU-R BT.601），最常用，最符合人眼感知
    Luminance,
    /// 加权平均（ITU-R BT.709），HD/现代显示器标准
    Luminance709,
    /// 简单平均法 (R + G + B) / 3
    Average,
    /// 最大值法，max(R, G, B)，结果偏亮
    Max,
    /// 最小值法，min(R, G, B)，结果偏暗
    Min,
    /// 去饱和法，(max + min) / 2，Photoshop 早期"去色"命令
    Desaturate,
}

impl GrayAlgorithm {
    /// 从字符串解析（兼容 JS 端传入），未匹配时回退到 Luminance
    fn from_str(s: &str) -> Self {
        match s {
            "luminance709" | "Luminance709" => GrayAlgorithm::Luminance709,
            "average" | "Average" => GrayAlgorithm::Average,
            "max" | "Max" => GrayAlgorithm::Max,
            "min" | "Min" => GrayAlgorithm::Min,
            "desaturate" | "Desaturate" => GrayAlgorithm::Desaturate,
            _ => GrayAlgorithm::Luminance,
        }
    }
}

/// 单像素应用灰度算法
#[inline]
fn apply_gray(r: u8, g: u8, b: u8, algo: GrayAlgorithm) -> u8 {
    match algo {
        GrayAlgorithm::Luminance => {
            (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8
        }
        GrayAlgorithm::Luminance709 => {
            (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32) as u8
        }
        GrayAlgorithm::Average => ((r as u16 + g as u16 + b as u16) / 3) as u8,
        GrayAlgorithm::Max => r.max(g).max(b),
        GrayAlgorithm::Min => r.min(g).min(b),
        GrayAlgorithm::Desaturate => {
            let max = r.max(g).max(b);
            let min = r.min(g).min(b);
            ((max as u16 + min as u16) / 2) as u8
        }
    }
}

/// 使用指定算法做灰度
///
/// `algorithm` 支持以下字符串：
/// - `"luminance"`     加权平均 BT.601（默认）
/// - `"luminance709"`  加权平均 BT.709
/// - `"average"`       简单平均
/// - `"max"`           最大值
/// - `"min"`           最小值
/// - `"desaturate"`    去饱和
#[wasm_bindgen]
pub fn grayscale_with(pixels: &[u8], width: u32, height: u32, algorithm: &str) -> Vec<u8> {
    let len = (width * height * 4) as usize;
    let algo = GrayAlgorithm::from_str(algorithm);
    let mut result = Vec::with_capacity(len);

    for i in (0..len).step_by(4) {
        let gray = apply_gray(pixels[i], pixels[i + 1], pixels[i + 2], algo);
        result.push(gray);
        result.push(gray);
        result.push(gray);
        result.push(pixels[i + 3]); // 保留 Alpha 通道
    }

    result
}
