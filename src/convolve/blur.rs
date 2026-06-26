use wasm_bindgen::prelude::*;

/// 均值模糊（3×3 邻域平均，重复 N 次）
///
/// 每次迭代对每个像素用周围 3×3 邻域的平均值替代。
/// N 次迭代的效果类似 (2N+1) × (2N+1) 的盒式模糊，
/// 但更柔和（中心极限定理 → 接近高斯）。
///
/// | iterations | 视觉效果                  |
/// | ---------- | ------------------------- |
/// | 1          | 3×3，**非常微弱**         |
/// | 2          | ≈ 5×5，轻微可见          |
/// | 3          | ≈ 7×7，明显模糊          |
/// | 5          | ≈ 11×11，重模糊          |
///
/// # 参数
///
/// - `pixels`:     RGBA 像素数组
/// - `width`:      图片宽度
/// - `height`:     图片高度
/// - `iterations`: 迭代次数（推荐 1~5）
///
/// # 返回
///
/// 模糊后的 RGBA 像素数组。
#[wasm_bindgen]
pub fn mean_blur(pixels: &[u8], width: u32, height: u32, iterations: u32) -> Vec<u8> {
    let mut result = pixels.to_vec();
    for _ in 0..iterations {
        result = mean_blur_once(&result, width, height);
    }
    result
}

/// 中值模糊（3×3 邻域取中位数，重复 N 次）
///
/// 每次迭代对每个像素用周围 3×3 邻域的中位数替代。
/// 对椒盐噪声特别有效，不像均值模糊会把噪声"扩散"。
///
/// # 参数
///
/// - `pixels`:     RGBA 像素数组
/// - `width`:      图片宽度
/// - `height`:     图片高度
/// - `iterations`: 迭代次数（推荐 1~3，太多会让图变"油画"）
///
/// # 返回
///
/// 模糊后的 RGBA 像素数组。
#[wasm_bindgen]
pub fn median_blur(pixels: &[u8], width: u32, height: u32, iterations: u32) -> Vec<u8> {
    let mut result = pixels.to_vec();
    for _ in 0..iterations {
        result = median_blur_once(&result, width, height);
    }
    result
}

// ========== 私有辅助函数（不导出到 JS） ==========

/// 单次 3×3 均值模糊
fn mean_blur_once(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let len = (width * height * 4) as usize;
    let mut result = vec![0u8; len];
    let w = width as i32;
    let h = height as i32;

    for y in 0..h {
        for x in 0..w {
            let mut sum_r: u32 = 0;
            let mut sum_g: u32 = 0;
            let mut sum_b: u32 = 0;
            let mut count: u32 = 0;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0 || nx >= w || ny < 0 || ny >= h {
                        continue;
                    }
                    let idx = ((ny * w + nx) * 4) as usize;
                    sum_r += pixels[idx] as u32;
                    sum_g += pixels[idx + 1] as u32;
                    sum_b += pixels[idx + 2] as u32;
                    count += 1;
                }
            }

            let out = ((y * w + x) * 4) as usize;
            result[out]     = (sum_r / count) as u8;
            result[out + 1] = (sum_g / count) as u8;
            result[out + 2] = (sum_b / count) as u8;
            result[out + 3] = pixels[out + 3];
        }
    }

    result
}

/// 单次 3×3 中值模糊
fn median_blur_once(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let len = (width * height * 4) as usize;
    let mut result = vec![0u8; len];
    let w = width as i32;
    let h = height as i32;

    for y in 0..h {
        for x in 0..w {
            let mut rs = [0u8; 9];
            let mut gs = [0u8; 9];
            let mut bs = [0u8; 9];
            let mut count = 0usize;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0 || nx >= w || ny < 0 || ny >= h {
                        continue;
                    }
                    let idx = ((ny * w + nx) * 4) as usize;
                    rs[count] = pixels[idx];
                    gs[count] = pixels[idx + 1];
                    bs[count] = pixels[idx + 2];
                    count += 1;
                }
            }

            rs[..count].sort();
            gs[..count].sort();
            bs[..count].sort();
            let mid = count / 2;

            let out = ((y * w + x) * 4) as usize;
            result[out]     = rs[mid];
            result[out + 1] = gs[mid];
            result[out + 2] = bs[mid];
            result[out + 3] = pixels[out + 3];
        }
    }

    result
}
