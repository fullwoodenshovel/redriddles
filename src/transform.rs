use std::{collections::HashSet, fmt::Debug, hash::Hash};
use macroquad::prelude::*;
use super::bresenham::Bresenham;

use super::colour::{ColType, Rgba};

#[derive(Clone, Copy, Debug, Default)]
pub struct WorldPos(pub f32, pub f32);

#[derive(Clone, Copy, Debug, Default)]
pub struct ScreenPos(pub f32, pub f32);

impl std::ops::Add for ScreenPos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Transform {
    pub offset: (f32, f32),
    scale: f32,
    pub window_dims: (f32, f32),
}


#[derive(Clone, Copy, Debug, Default)]
pub struct Pixel {
    pub pos: [i16; 2],
    pub col: [f32; 4]
}

impl Hash for Pixel {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for Pixel {}

#[derive(Clone, Debug)]
pub struct PixelArray {
    pixels: HashSet<Pixel>,
    pub grid_col: [f32; 4],
    pub crossboard_col: [f32; 4]
}

fn try_from_f32_to_i16(float: f32) -> Option<i16> {
    if !(i16::MIN as f32..(i16::MAX as i32 + 1) as f32).contains(&float) { return None; }
    if float >= 0.0 {
        Some(float as i16)
    } else {
        Some(((float - (i16::MIN as f32)) as i32 + i16::MIN as i32) as i16)
    }
}

impl Pixel {
    pub fn from_f32(x: f32, y: f32, col: [f32; 4]) -> Option<Pixel> {
        Some(Pixel {
            pos: [
                try_from_f32_to_i16(x)?,
                try_from_f32_to_i16(y)?
            ],
            col
        })
    }
}

impl Transform {
    pub fn new(window_dims: (f32, f32)) -> Self {
        Self {
            offset: (0.0, 0.0),
            scale: 10.0,
            window_dims,
        }
    }

    /// Returns the width / height of one pixel
    pub fn size(&self) -> f32 {
        self.scale
    }

    pub fn world_to_screen(&self, world: &WorldPos) -> ScreenPos {
        ScreenPos(
            world.0 * self.scale + self.offset.0,
            world.1 * self.scale + self.offset.1
        )
    }

    pub fn world_to_screen_filter(&self, world: &WorldPos) -> Option<ScreenPos> {
        let result = self.world_to_screen(world);

        if self.in_screen(&result) {
            Some(result)
        } else {
            None
        }
    }

    pub fn in_screen(&self, pos: &ScreenPos) -> bool {
        (-self.scale <= pos.0) && (pos.0 < self.window_dims.0) &&
        (-self.scale <= pos.1) && (pos.1 < self.window_dims.1)
    }

    pub fn screen_to_world(&self, screen: &ScreenPos) -> WorldPos {
        WorldPos(
            (screen.0 - self.offset.0) / self.scale,
            (screen.1 - self.offset.1) / self.scale,
        )
    }

    pub fn scale_about(&mut self, mut scale: f32, pos: ScreenPos, min: f32, max: f32) {
        let mut new = self.scale * scale;

        if new > max {
            new = max;
            scale = new / self.scale
        } else if new < min {
            new = min;
            scale = new / self.scale
        }

        self.scale = new;
        self.offset.0 = pos.0 - (pos.0 - self.offset.0) * scale;
        self.offset.1 = pos.1 - (pos.1 - self.offset.1) * scale;
    }

    pub fn get_int_pos(&self, pos: Vec2) -> Option<[i16; 2]> {
        let screen = ScreenPos(pos.x, pos.y);
        let world = self.screen_to_world(&screen);
        let world = world.as_i16()?;
        Some(world)
    }
}

impl Default for PixelArray {
    fn default() -> Self {
        let grid_col = Rgba::from_hex(0xf7a5ca).to_rgba();
        let grid_col = [grid_col[0], grid_col[1], grid_col[2], 0.5];

        let crossboard_col = Rgba::from_hex(0x000000).to_rgba();
        let crossboard_col = [crossboard_col[0], crossboard_col[1], crossboard_col[2], 0.07];

        Self {
            pixels: HashSet::new(),
            grid_col,
            crossboard_col
        }
    }
}

impl PixelArray {
    pub fn insert(&mut self, pixel: Pixel) {
        self.pixels.take(&pixel);
        self.pixels.insert(pixel);
    }

    pub fn remove(&mut self, pos: [i16; 2]) {
        self.pixels.take(&Pixel { pos, col: [0.0; 4] });
    }

    pub fn get(&self, pos: [i16; 2]) -> Option<&Pixel> {
        self.pixels.get(&Pixel { pos, col: [0.0; 4] })
    }

    pub fn get_at_mouse(&self, pos: Vec2, transform: &Transform) -> Option<&Pixel> {
        let pos = transform.get_int_pos(pos)?;
        self.get(pos)
    }

    pub fn draw(&self, transform: &Transform, grid_lines: bool, crossboard: bool) {
        if crossboard {
            self.draw_crossboard(transform)
        }

        for pixel in &self.pixels {
            let Some(pos) = transform.world_to_screen_filter(&WorldPos(pixel.pos[0] as f32, pixel.pos[1] as f32)) else { continue };
            let size = transform.size();
            draw_rectangle(
                pos.0,
                pos.1,
                size,
                size,
                Color::new(
                    pixel.col[0],
                    pixel.col[1],
                    pixel.col[2],
                    pixel.col[3],
                ),
            );
        }
        if grid_lines {
            self.draw_grid_lines(transform)
        }
    }

    pub fn draw_grid_lines(&self, transform: &Transform) {
        let start = transform.screen_to_world(&ScreenPos(0.0, 0.0));
        let start = WorldPos(
            match try_from_f32_to_i16(start.0) {
                Some(x) => x as f32,
                None => return
            },
            match try_from_f32_to_i16(start.1) {
                Some(x) => x as f32,
                None => return
            });
        let start = transform.world_to_screen(&start);

        let Some(nx) = try_from_f32_to_i16(transform.window_dims.0 / transform.scale) else { return; };
        let Some(ny) = try_from_f32_to_i16(transform.window_dims.1 / transform.scale) else { return; };

        let nx = nx + 2;
        let ny = ny + 2;

        for i in 0..nx {
            let pos = ScreenPos(i as f32 * transform.scale, 0.0) + start;
            let sizex = 1.0;
            let sizey = transform.window_dims.1 + transform.scale;
            draw_rectangle(
                pos.0,
                pos.1,
                sizex,
                sizey,
                Color::new(
                    self.grid_col[0],
                    self.grid_col[1],
                    self.grid_col[2],
                    self.grid_col[3]
                )
            );
        };

        for i in 0..ny {
            let pos = ScreenPos(0.0, i as f32 * transform.scale) + start;
            let sizex = transform.window_dims.0 + transform.scale;
            let sizey = 1.0;

            draw_rectangle(
                pos.0,
                pos.1,
                sizex,
                sizey,
                Color::new(
                    self.grid_col[0],
                    self.grid_col[1],
                    self.grid_col[2],
                    self.grid_col[3]
                )
            );
        }
    }

    pub fn draw_crossboard(&self, transform: &Transform) {
        let start = transform.screen_to_world(&ScreenPos(0.0, 0.0));
        let start = WorldPos(
            match try_from_f32_to_i16(start.0) {
                Some(x) => x as f32,
                None => return
            },
            match try_from_f32_to_i16(start.1) {
                Some(y) => y as f32,
                None => return
            });
        let parity = ((start.0 as i64 + start.1 as i64) % 2) as i16;
        let start = transform.world_to_screen(&start);
        
        let Some(nx) = try_from_f32_to_i16(transform.window_dims.0 / transform.scale) else { return; };
        let Some(ny) = try_from_f32_to_i16(transform.window_dims.1 / transform.scale) else { return; };

        let nx = nx + 2;
        let ny = ny/2 + 2;

        for x in 0..nx {
            for y in 0..ny {
                let y = y * 2 + x % 2 - parity;
                let pos = ScreenPos(x as f32 * transform.scale, y as f32 * transform.scale) + start;
                draw_rectangle(
                    pos.0,
                    pos.1,
                    transform.scale,
                    transform.scale,
                    Color::new(
                        self.crossboard_col[0],
                        self.crossboard_col[1],
                        self.crossboard_col[2],
                        self.crossboard_col[3]
                    )
                );
            }
        };
    }

    pub fn get_bounds(&self) -> [WorldPos; 2] { // top left and bottom right corner
        let mut iter = self.pixels.iter();
        let Some(pixel) = iter.next() else {
            return [WorldPos(-1.0, -1.0), WorldPos(1.0, 1.0)]
        };

        let mut minx = pixel.pos[0];
        let mut miny = pixel.pos[1];
        let mut maxx = pixel.pos[0];
        let mut maxy = pixel.pos[1];
        
        for pixel in iter {
            let [x, y] = pixel.pos;
            if minx > x {
                minx = x;
            } else if maxx < x {
                maxx = x
            }
            if miny > y {
                miny = y;
            } else if maxy < y {
                maxy = y
            }
        }

        [
            WorldPos(minx as f32, miny as f32),
            WorldPos(maxx as f32, maxy as f32)
        ]
    }

    pub fn fill(&mut self, pos: [i16; 2], col: Option<[f32; 4]>) {
        let [WorldPos(minx, miny), WorldPos(maxx, maxy)] = self.get_bounds();
        let (minx, miny, maxx, maxy) = (minx as i16, miny as i16, maxx as i16, maxy as i16);
        let old_col = self.get(pos).map(|p| p.col);

        let mut additions = HashSet::new();
        let mut unchecked = vec![];
        let mut checked = HashSet::new();

        unchecked.push(pos);

        while let Some(pos) = unchecked.pop() {
            let [x, y] = pos;
            if minx > x || maxx < x || miny > y || maxy < y {            
                additions = HashSet::new();
                break
            }
            
            for pos in [[x+1, y], [x, y+1], [x-1, y], [x, y-1]] {
                if !checked.contains(&pos) && self.get(pos).map(|p| p.col) == old_col {
                    unchecked.push(pos);
                }
            }
            
            additions.insert(pos);

            checked.insert(pos);
        }

        match col {
            Some(col) => {
                for pos in additions {
                    self.insert(Pixel { pos, col });
                }
            },
            None => {
                for pos in additions {
                    self.remove(pos);
                }
            }
        }
    }

    pub fn line(&mut self, start: [i16; 2], end: [i16; 2], col: Option<[f32; 4]>) {
        match col {
            Some(col) => {
                for pos in Bresenham::new(start, end) {
                    self.insert(Pixel { pos, col });
                }
            },
            None => {
                for pos in Bresenham::new(start, end) {
                    self.remove(pos)
                }
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Pixel> {
        self.pixels.iter()
    }
}

impl WorldPos {
    pub fn as_i16(&self) -> Option<[i16; 2]> {
        Some([
            try_from_f32_to_i16(self.0)?,
            try_from_f32_to_i16(self.1)?
        ])
    }
}