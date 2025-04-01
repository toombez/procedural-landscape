use noise::{NoiseFn, Perlin, Seedable};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct LandscapeGenerator {
    perlin: Perlin,
}

#[wasm_bindgen]
impl LandscapeGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u32) -> Self {
        Self {
            perlin: Perlin::new(seed),
        }
    }

    #[wasm_bindgen]
    pub fn generate_heightmap(&self, width: usize, height: usize, scale: f64) -> Vec<f32> {
        let mut heightmap = Vec::with_capacity(width * height);

        for y in 0..height {
            for x in 0..width {
                let nx = x as f64 / width as f64 * scale;
                let ny = y as f64 / height as f64 * scale;
                let value = self.perlin.get([nx, ny]);
                heightmap.push(value as f32);
            }
        }

        heightmap
    }
}
