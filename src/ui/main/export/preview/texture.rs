use super::*;

pub struct RawTexture {
    pub texture: Vec<u8>,
    pub width: u16,
    pub height: u16,
    pub average: [f32; 4],
    // pub noise: f32
}

impl RawTexture {
    pub fn new(width: u16, height: u16, pixels: Vec<u8>, averaging_col: ColSelection) -> Self {
        Self {
            average: get_average(&pixels, averaging_col),
            width,
            height,
            texture: pixels
        }
    }
}

fn get_average(texture: &[u8], col_sel: ColSelection) -> [f32; 4] {
    let mut sx = 0.0;
    let mut sy = 0.0;
    let mut sz = 0.0;
    let mut sa = 0.0;
    let mut all_full = true;

    let count = (texture.len() / 4 ) as f32;

    let chunks = texture.as_chunks();
    if !chunks.1.is_empty() {
        panic!("When loading texture, the length of the subpixels isnt a multiple of 4");
    }

    let chunks = chunks.0;
    for [r, g, b, a] in chunks.iter() {
        if *a != u8::MAX {
            all_full = false;
        }
        let fa = *a as f32 / 255.0;
        let col = [
            *r as f32 / 255.0,
            *g as f32 / 255.0,
            *b as f32 / 255.0,
            fa,
        ];

        sa += col[3];

        let col = col_sel.col_from_rgba_arr(col);

        let col = col.to_wheel();

        sx += col.0 * fa;
        sy += col.1 * fa;
        sz += col.2 * fa;
    }

    sx /= count;
    sy /= count;
    sz /= count;
    sa /= count;
    
    let mut result = col_sel.col_from_wheel(sx, sy, sz).to_rgba();
    result[3] = if all_full {1.0} else {sa};
    result
}