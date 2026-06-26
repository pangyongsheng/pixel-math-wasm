use wasm_bindgen::prelude::*;

/// 马赛克像素化
///
/// 把图片分成 `block_size × block_size` 的小块，每块取**平均色**填充整块。
///
/// | `block_size` | 视觉效果              |
/// | ------------ | --------------------- |
/// | 5            | 轻微像素化            |
/// | 10           | 标准马赛克 ⭐         |
/// | 20           | 抽象艺术              |
/// | 50           | 色块拼贴              |
///
/// # 参数
///
/// - `pixels`:     RGBA 像素数组
/// - `width`:      图片宽度
/// - `height`:     图片高度
/// - `block_size`: 马赛克块大小（正方形边长，像素）
///
/// # 返回
///
/// 马赛克处理后的 RGBA 像素数组。
#[wasm_bindgen]
pub fn mosaic(pixels: &[u8], width: u32, height: u32, block_size: u32) -> Vec<u8> {
    let len = (width * height * 4) as usize;
    let mut result = vec![0u8; len];
    let w = width as i32;
    let h = height as i32;
    let bs = block_size as i32;

    // 按 block_size 步长遍历（左上角起点）
    for by in (0..h).step_by(bs as usize) {
        for bx in (0..w).step_by(bs as usize) {
            // 1. 先算这块的平均色
            let mut sum_r: u32 = 0;
            let mut sum_g: u32 = 0;
            let mut sum_b: u32 = 0;
            let mut count: u32 = 0;

            let y_end = (by + bs).min(h);
            let x_end = (bx + bs).min(w);

            for y in by..y_end {
                for x in bx..x_end {
                    let i = ((y * w + x) * 4) as usize;
                    sum_r += pixels[i] as u32;
                    sum_g += pixels[i + 1] as u32;
                    sum_b += pixels[i + 2] as u32;
                    count += 1;
                }
            }

            let avg_r = (sum_r / count) as u8;
            let avg_g = (sum_g / count) as u8;
            let avg_b = (sum_b / count) as u8;

            // 2. 把整块都涂成这个平均色
            for y in by..y_end {
                for x in bx..x_end {
                    let i = ((y * w + x) * 4) as usize;
                    result[i]     = avg_r;
                    result[i + 1] = avg_g;
                    result[i + 2] = avg_b;
                    result[i + 3] = pixels[i + 3]; // 保留 Alpha
                }
            }
        }
    }

    result
}
