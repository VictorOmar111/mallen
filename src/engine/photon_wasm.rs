//! Kernel Matemático WebAssembly (wasm32) para interpolación vectorial y Spatial Culling a 120 FPS.
#![no_std]

#[no_mangle]
pub extern "C" fn lerp(current: f32, target: f32, factor: f32) -> f32 {
    current + (target - current) * factor
}

#[no_mangle]
pub extern "C" fn dist_sq(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    dx * dx + dy * dy
}
