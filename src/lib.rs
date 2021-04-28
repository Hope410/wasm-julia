mod utils;
use num_complex::Complex;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::CanvasRenderingContext2d;
use web_sys::ImageData;
use js_sys::Uint8ClampedArray;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Range(pub f64, pub f64);

#[wasm_bindgen]
impl Range {
    pub fn new(a: f64, b: f64) -> Range {
        Range(a, b)
    }
}

#[wasm_bindgen]
pub struct Julia {
    ctx: CanvasRenderingContext2d,
    width: u32,
    height: u32,
    iterations: u32,
    threshold: f64,
    colors: Uint8ClampedArray,
}

#[wasm_bindgen]
impl Julia {
    pub fn new(
        ctx: CanvasRenderingContext2d,
        width: u32,
        height: u32,
        iterations: u32,
        threshold: f64,
        colors: Uint8ClampedArray,
    ) -> Julia {
        Julia {
            ctx,
            width,
            height,
            iterations,
            threshold,
            colors,
        }
    }

    pub fn draw(&self, range_x: Range, range_y: Range, real: f64, imaginary: f64) -> Result<(), JsValue> {
        let complex = Complex::new(real, imaginary);
        let mut data = self.compute_set(range_x, range_y, complex);

        let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), self.width, self.height)?;
        self.ctx.put_image_data(&data, 0.0, 0.0)
    }

    fn compute_set(&self, range_x: Range, range_y: Range, complex: Complex<f64>) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();

        let range_width = range_x.1 - range_x.0;
        let scale_x = self.width as f64 / range_width;

        let range_height = range_y.1 - range_y.0;
        let scale_y = self.height as f64 / range_height;
        for x in 0..self.width {
            for y in 0..self.height {
                let z = Complex::new(
                    x as f64 / scale_x + range_x.0,
                    y as f64 / scale_y + range_y.0,
                );

                let iter_index = self.get_iter_index(z, complex);
                let iter_index = iter_index * 4;

                data.push(self.colors.get_index(iter_index));
                data.push(self.colors.get_index(iter_index + 1));
                data.push(self.colors.get_index(iter_index + 2));
                data.push(self.colors.get_index(iter_index + 3));
            }
        }

        return data;
    }

    fn get_iter_index(&self, z: Complex<f64>, c: Complex<f64>) -> u32 {
        let mut iter_index: u32 = 0;
        let mut z = z;
        while iter_index < self.iterations {
            if z.norm() > self.threshold {
                break;
            }

            z = z * z + c;
            iter_index += 1;
        }
        iter_index
    }
}
