use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn chroma_key(data: &mut [u8], width: usize, height: usize, base_color: u32, radius: u32, max_distance: u32, blink_blue: bool) {
    let base_r = ((base_color >> 16) & 0xFF) as i32;
    let base_g = ((base_color >> 8) & 0xFF) as i32;
    let base_b = (base_color & 0xFF) as i32;

    for y in 0..height {
        for x in 0..width {
            let index = (y * width + x) * 4;
            let r = data[index] as i32;
            let g = data[index + 1] as i32;
            let b = data[index + 2] as i32;

            let r_diff = r - base_r;
            let g_diff = g - base_g;
            let b_diff = b - base_b;
            let distance = ((r_diff * r_diff + g_diff * g_diff + b_diff * b_diff) as f64).sqrt() as u32;

            if distance < radius {
                if blink_blue {
                    data[index] = 0;
                    data[index + 1] = 0;
                    data[index + 2] = 255;
                } else {
                    data[index + 3] = 0;
                }
            } else if distance < max_distance {
                let alpha_factor = (max_distance - distance) as f64 / (max_distance - radius) as f64;
                data[index + 3] = (data[index + 3] as f64 * alpha_factor) as u8;
            }
        }
    }
}
