use nalgebra::Matrix3;
use wasm_bindgen::prelude::*;

/// 浮雕/雕刻卷积核
///
/// ```
/// -2  -1   0
/// -1   1   1
///  0   1   2
/// ```
///
/// 本质：对每个像素算"右下邻居 - 左上邻居"。
/// 平坦区域结果 ≈ 0，加上 128 偏置后 = 中灰 → 看起来"无变化"。
/// 边缘区域结果 ≠ 0，呈现亮（凸起）或暗（凹陷）。
fn emboss_kernel() -> Matrix3<i32> {
    Matrix3::from_row_slice(&[
        -2, -1,  0,
        -1,  1,  1,
         0,  1,  2,
    ])
}

/// 浮雕/雕刻滤镜
///
/// 对每个像素：
/// 1. 用 3×3 卷积核算"右下 - 左上"差值
/// 2. 加上 128 偏置，让结果居中（平 = 128，凸 = 亮，凹 = 暗）
/// 3. 截断到 0~255
///
/// # 参数
///
/// - `pixels`: RGBA 像素数组
/// - `width`:  图片宽度
/// - `height`: 图片高度
///
/// # 返回
///
/// 浮雕处理后的 RGBA 像素数组。
#[wasm_bindgen]
pub fn emboss(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let len = (width * height * 4) as usize;
    let mut result = vec![0u8; len];
    let w = width as i32;
    let h = height as i32;

    let kernel = emboss_kernel();

    for y in 0..h {
        for x in 0..w {
            // 1. 收集 3×3 邻域灰度
            let neighborhood: Matrix3<i32> = Matrix3::from_row_slice(&[
                gray_at(pixels, w, h, x - 1, y - 1),
                gray_at(pixels, w, h, x,     y - 1),
                gray_at(pixels, w, h, x + 1, y - 1),
                gray_at(pixels, w, h, x - 1, y    ),
                gray_at(pixels, w, h, x,     y    ),
                gray_at(pixels, w, h, x + 1, y    ),
                gray_at(pixels, w, h, x - 1, y + 1),
                gray_at(pixels, w, h, x,     y + 1),
                gray_at(pixels, w, h, x + 1, y + 1),
            ]);

            // 2. 卷积：与核的对应元素相乘再求和
            let sum: i32 = neighborhood.iter()
                .zip(kernel.iter())
                .map(|(a, b)| a * b)
                .sum();

            // 3. 加 128 偏置 + clamp
            let embossed = (sum + 128).clamp(0, 255) as u8;

            // 4. 灰度化的浮雕：3 通道都填这个值
            let out = ((y * w + x) * 4) as usize;
            result[out]     = embossed;
            result[out + 1] = embossed;
            result[out + 2] = embossed;
            result[out + 3] = pixels[out + 3]; // alpha
        }
    }

    result
}

/// 取一个像素的 BT.709 灰度（越界返回 0）
fn gray_at(pixels: &[u8], w: i32, h: i32, x: i32, y: i32) -> i32 {
    if x < 0 || x >= w || y < 0 || y >= h {
        return 0;
    }
    let i = ((y * w + x) * 4) as usize;
    (0.2126 * pixels[i] as f32
   + 0.7152 * pixels[i + 1] as f32
   + 0.0722 * pixels[i + 2] as f32) as i32
}
