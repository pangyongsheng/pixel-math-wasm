use nalgebra::Matrix3;
use wasm_bindgen::prelude::*;

/// Sobel 卷积核（懒加载，避免 const 限制）
fn sobel_x() -> Matrix3<i32> {
    Matrix3::from_row_slice(&[
        -1, 0, 1,
        -2, 0, 2,
        -1, 0, 1,
    ])
}

/// Sobel 卷积核 Gy（检测垂直边缘）
fn sobel_y() -> Matrix3<i32> {
    Matrix3::from_row_slice(&[
        -1, -2, -1,
         0,  0,  0,
         1,  2,  1,
    ])
}

/// Sobel 边缘检测
///
/// 对每个像素：
/// 1. 取 3×3 邻域的灰度
/// 2. 与 Gx / Gy 核做"卷积"（矩阵元素相乘后求和）
/// 3. 梯度幅值 = |Gx| + |Gy|
/// 4. 超过阈值 → 白（边缘），否则 → 黑
///
/// # 参数
///
/// - `pixels`:    RGBA 像素数组
/// - `width`:     图片宽度
/// - `height`:    图片高度
/// - `threshold`: 边缘阈值 0~255（越大越严格）
///
/// # 返回
///
/// 二值化边缘图（白=边缘，黑=背景）。
#[wasm_bindgen]
pub fn sobel(pixels: &[u8], width: u32, height: u32, threshold: i32) -> Vec<u8> {
    let len = (width * height * 4) as usize;
    let mut result = vec![0u8; len];
    let w = width as i32;
    let h = height as i32;

    // 核矩阵（每次函数调用创建一次，开销可忽略）
    let kx = sobel_x();
    let ky = sobel_y();

    for y in 0..h {
        for x in 0..w {
            // 1. 收集 3×3 邻域灰度（用 Matrix3 存，跟核矩阵同结构）
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

            // 2. 与两个核做"卷积"：对应元素相乘再求和
            let gx: i32 = neighborhood.iter()
                .zip(kx.iter())
                .map(|(a, b)| a * b)
                .sum();
            let gy: i32 = neighborhood.iter()
                .zip(ky.iter())
                .map(|(a, b)| a * b)
                .sum();

            // 3. 梯度幅值 + 阈值化
            let magnitude = (gx.abs() + gy.abs()).min(255) as u8;
            let edge = if magnitude as i32 > threshold { 255 } else { 0 };

            // 4. 输出（白边或黑背景，alpha 保留）
            let out = ((y * w + x) * 4) as usize;
            result[out]     = edge;
            result[out + 1] = edge;
            result[out + 2] = edge;
            result[out + 3] = pixels[out + 3];
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
