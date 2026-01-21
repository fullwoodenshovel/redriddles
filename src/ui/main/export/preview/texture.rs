use super::*;

pub struct RawTexture {
    pub texture: Vec<u8>,
    pub width: u16,
    pub height: u16,
    pub average: [f32; 4],
    // pub noise: f32
}

impl RawTexture {
    pub fn new(width: u16, height: u16, pixels: Vec<u8>, col_sel: ColSelection) -> Self {
        Self {
            average: get_average(&pixels, col_sel),
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

    let count = (texture.len() / 4 )as f32;

    let chunks = texture.as_chunks();
    if !chunks.1.is_empty() {
        panic!("When loading texture, the length of the subpixels isnt a multiple of 4");
    }

    let chunks = chunks.0;
    for [r, g, b, a] in chunks.iter() {
        let col = [
            *r as f32 / 255.0,
            *g as f32 / 255.0,
            *b as f32 / 255.0,
            *a as f32 / 255.0,
        ];

        sa += col[3];

        let col = col_sel.col_from_rgba_arr(col);

        let col = col.to_wheel();

        sx += col.0;
        sy += col.1;
        sz += col.2;
    }

    sx /= count;
    sy /= count;
    sz /= count;

    let mut result = col_sel.col_from_wheel(sx, sy, sz).to_rgba();
    result[3] = sa;
    result
}