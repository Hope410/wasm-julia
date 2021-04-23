mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet(ctx: &CanvasRenderingContext2d, name: &str) {
    ctx.set_font("50px serif");
    ctx.fill_text(format!("Hello, {}!", name).as_str(), 0.0, 50.0)
        .expect("Can't draw ctx message!");
}