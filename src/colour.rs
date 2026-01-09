use std::fmt::Debug;

use macroquad::{color::Color, math::{Vec3, Vec4}};

use oklab::{Oklab, Rgb, oklab_to_srgb_f32, srgb_f32_to_oklab};

use crate::helpers::arr_to_macroquad;

pub enum Col {
    Rgba(Rgba),
    Hsva(Hsva),
    OkLab(OkLab),
}

#[derive(PartialEq, Hash, Eq, Clone, Copy)]
pub enum ColSelection {
    Rgba,
    Hsva,
    OkLab,
}

impl Col {
    pub fn as_dyn(&self) -> Box<dyn ColType> {
        match self {
            Self::Rgba(col) => Box::new(*col) as Box<dyn ColType>,
            Self::Hsva(col) => Box::new(*col) as Box<dyn ColType>,
            Self::OkLab(col) => Box::new(*col) as Box<dyn ColType>,
        }
    }

    pub fn to_rgba(&self) -> [f32; 4] {
        self.as_dyn().to_rgba()
    }

    pub fn distance(&self, other: [f32; 4]) -> f32 {
        self.as_dyn().distance(other)
    }
    
    pub fn to_wheel(&self) -> (f32, f32, f32) {
        self.as_dyn().to_wheel()
    }
    
    pub fn to_macroquad_col(&self) -> Color {
        arr_to_macroquad(self.to_rgba())
    }

    pub fn to_rgba_u8(&self) -> [u8; 4] {
        self.to_rgba().map(|d| (d * 255.0) as u8)
    }

    pub fn to_col_sel(&self) -> ColSelection {
        match self {
            Self::Rgba(_) => ColSelection::Rgba,
            Self::Hsva(_) => ColSelection::Hsva,
            Self::OkLab(_) => ColSelection::OkLab,
        }
    }

    pub fn to_hex_string(&self) -> String {
        let rgb = self.to_rgba_u8();
        ColSelection::format_rgba_u8(rgb)
    }
}

impl ColSelection {
    pub fn format_rgba_u8(arr: [u8; 4]) -> String {
        format!("#{:02X}{:02X}{:02X}", arr[0], arr[1], arr[2])
    }

    pub fn format_rgba(arr: [f32; 4]) -> String {
        Self::format_rgba_u8(arr.map(|d| (d * 255.0) as u8))
    }

    pub fn col_from_rgba(&self, r: f32, g: f32, b: f32, a: f32) -> Col {
        match self {
            Self::Rgba => Col::Rgba(Rgba::from_rgba(r, g, b, a)),
            Self::Hsva => Col::Hsva(Hsva::from_rgba(r, g, b, a)),
            Self::OkLab => Col::OkLab(OkLab::from_rgba(r, g, b, a)),
        }
    }
    
    pub fn col_from_wheel(&self, circular: f32, radial: f32, scalar: f32) -> Col {
        match self {
            Self::Rgba => Col::Rgba(Rgba::from_wheel(circular, radial, scalar)),
            Self::Hsva => Col::Hsva(Hsva::from_wheel(circular, radial, scalar)),
            Self::OkLab => Col::OkLab(OkLab::from_wheel(circular, radial, scalar)),
        }
    }
    
    pub fn col_from_rgba_u8(&self, r: u8, g: u8, b: u8, a: u8) -> Col {
        self.col_from_rgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0)
    }
    
    pub fn col_from_rgba_arr(&self, arr: [f32; 4]) -> Col {
        self.col_from_rgba(arr[0], arr[1], arr[2], arr[3])
    }

    pub fn col_from_rgba_hex(&self, hex: u32) -> Col {
        let [r, g, b, a] = hex.to_be_bytes();
        self.col_from_rgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0)
    }

    pub fn col_from_rgb_hex(&self, hex: u32) -> Col {
        self.col_from_rgba_hex((hex << 8) + 0xFF)
    }

    /// Panics if string isnt only 0-9, a-z, A-Z
    pub fn col_from_hex_string(&self, hex: &str) -> Col {
        self.col_from_rgb_hex(u32::from_str_radix(hex, 16).unwrap())
    }

    pub fn default_cirular(&self) -> f32 {
        match self {
            Self::Rgba => Rgba::default_circular(),
            Self::Hsva => Hsva::default_circular(),
            Self::OkLab => OkLab::default_circular(),
        }
    }
    
    pub fn default_radial(&self) -> f32 {
        match self {
            Self::Rgba => Rgba::default_radial(),
            Self::Hsva => Hsva::default_radial(),
            Self::OkLab => OkLab::default_radial(),
        }
    }

    pub fn default_scalar(&self) -> f32 {
        match self {
            Self::Rgba => Rgba::default_scalar(),
            Self::Hsva => Hsva::default_scalar(),
            Self::OkLab => OkLab::default_scalar(),
        }
    }

}

pub trait ColType {
    fn default_circular() -> f32 where Self: Sized;
    fn default_radial() -> f32 where Self: Sized;
    fn default_scalar() -> f32 where Self: Sized;

    fn to_rgba(&self) -> [f32; 4];
    fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self where Self: Sized;
    fn distance(&self, other: [f32; 4]) -> f32;
    fn from_wheel(circular: f32, radial: f32, scalar: f32) -> Self where Self: Sized;
    fn to_wheel(&self) -> (f32, f32, f32);
    fn gradient(&self, other: [f32; 4], start_index: u8, end_index: u8, len: u8) -> Vec<Self> where Self: Sized;

    fn from_rgba_arr(arr: [f32; 4]) -> Self where Self: std::marker::Sized {
        Self::from_rgba(arr[0], arr[1], arr[2], arr[3])
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Rgba {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}

impl Rgba {
    pub fn from_hex(hex: u32) -> Self {
        let result = macroquad::color::Color::from_hex(hex);

        Self {
            r: result.r,
            g: result.g,
            b: result.b,
            a: result.a
        }
    }

    pub fn to_vec(self) -> Vec4 {
        Vec4 { x: self.r, y: self.g, z: self.b, w: self.a }
    }
}

impl ColType for Rgba {
    fn default_circular() -> f32 {0.0}
    fn default_radial() -> f32 {0.0}
    fn default_scalar() -> f32 {0.0}

    fn to_rgba(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
    
    fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    
    fn distance(&self, other: [f32; 4]) -> f32 {
        self.to_vec().distance_squared(Self::from_rgba_arr(other).to_vec())
    }
    
    fn from_wheel(circular: f32, radial: f32, scalar: f32) -> Self {
        Self {
            r: circular,
            g: radial,
            b: scalar,
            a: 1.0
        }
    }

    fn to_wheel(&self) -> (f32, f32, f32) {
        (self.r, self.g, self.b)
    }

    fn gradient(&self, other: [f32; 4], start_index: u8, end_index: u8, len: u8) -> Vec<Self> {
        let vec = self.to_vec();
        let other = Vec4::from_array(other);

        let change = (other - vec) / (end_index - start_index) as f32;
        let initial = vec - change * start_index as f32;

        let mut result = vec![];

        for i in 0..len {
            let current = initial + change * i as f32;
            result.push(Self::from_rgba_arr(current.to_array()));
        }

        result
    }
}

#[derive(Clone, Copy, Default)]
pub struct Hsva {
    h: f32,
    s: f32,
    v: f32,
    a: f32
}

impl ColType for Hsva {
    fn default_circular() -> f32 {0.0}
    fn default_radial() -> f32 {1.0}
    fn default_scalar() -> f32 {1.0}

    fn to_rgba(&self) -> [f32; 4] {
        // Wrap hue, clamp others
        let h = self.h.rem_euclid(1.0);
        let s = self.s.clamp(0.0, 1.0);
        let v = self.v.clamp(0.0, 1.0);
        let a = self.a.clamp(0.0, 1.0);

        let h = h * 360.0;

        let c = v * s;
        let h_prime = h / 60.0;
        let x = c * (1.0 - (h_prime.rem_euclid(2.0) - 1.0).abs());
        let m = v - c;

        let (r, g, b) = match h_prime {
            hp if hp < 1.0 => (c, x, 0.0),
            hp if hp < 2.0 => (x, c, 0.0),
            hp if hp < 3.0 => (0.0, c, x),
            hp if hp < 4.0 => (0.0, x, c),
            hp if hp < 5.0 => (x, 0.0, c),
            _                   => (c, 0.0, x),
        };

        [r + m, g + m, b + m, a]
    }

    fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        let r = r.clamp(0.0, 1.0);
        let g = g.clamp(0.0, 1.0);
        let b = b.clamp(0.0, 1.0);
        let a = a.clamp(0.0, 1.0);

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let v = max;

        let s = if max > 0.0 {
            delta / max
        } else {
            0.0
        };

        let mut h = if delta == 0.0 {
            0.0
        } else if max == r {
            (g - b) / delta
        } else if max == g {
            (b - r) / delta + 2.0
        } else {
            (r - g) / delta + 4.0
        };

        h = (h / 6.0).rem_euclid(1.0);

        Self { h, s, v, a }
    }

    fn distance(&self, other: [f32; 4]) -> f32 {
        let other = Self::from_rgba_arr(other);

        let x1 = self.v * self.s * f32::cos(self.h * std::f32::consts::TAU);
        let y1 = self.v * self.s * f32::sin(self.h * std::f32::consts::TAU);
        let z1 = self.v * 0.8;

        let x2 = other.v * other.s * f32::cos(other.h * std::f32::consts::TAU);
        let y2 = other.v * other.s * f32::sin(other.h * std::f32::consts::TAU);
        let z2 = other.v * 0.8;

        Vec3::new(x1, y1, z1).distance_squared(Vec3::new(x2, y2, z2))
    }

    fn from_wheel(circular: f32, radial: f32, scalar: f32) -> Self {
        Self {
            h: circular,
            s: radial,
            v: scalar,
            a: 1.0
        }
    }

    fn to_wheel(&self) -> (f32, f32, f32) {
        (self.h, self.s, self.v)
    }

    fn gradient(&self, other: [f32; 4], start_index: u8, end_index: u8, len: u8) -> Vec<Self> where Self: Sized {
        todo!();
        vec![Self::default(); len as usize]
    }
}

impl Debug for Hsva {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hsva {{ h: {}°, s: {}%, v: {}%, a: {}%}}",
            (self.h * 360.0) as u16,
            (self.s * 100.0) as u16,
            (self.v * 100.0) as u16,
            (self.a * 100.0) as u16
        )
    }
}



#[derive(Clone, Copy)]
pub struct OkLab {
    oklab: Oklab,
    a: f32
}

impl ColType for OkLab {
    fn default_circular() -> f32 {0.0}
    fn default_radial() -> f32 {0.0}
    fn default_scalar() -> f32 {0.0}

    fn to_rgba(&self) -> [f32; 4] {
        let rgb = oklab_to_srgb_f32(self.oklab);
        [rgb.r, rgb.g, rgb.b, self.a]
    }

    fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        let oklab = srgb_f32_to_oklab(Rgb { r, g, b });

        Self { oklab, a }
    }

    fn distance(&self, other: [f32; 4]) -> f32 {
        let other = srgb_f32_to_oklab(Rgb { r: other[0], g: other[1], b: other[2] });

        Vec3::new(self.oklab.l, self.oklab.a, self.oklab.b).distance_squared(Vec3::new(other.l, other.a, other.b))
    }

    fn from_wheel(circular: f32, radial: f32, scalar: f32) -> Self {
        // 0 < l <= 1
        // -0.234 < a < 0.277
        // -0.312 < b < 0.199
        Self {
            oklab: Oklab { l: scalar, a: circular * (0.234 + 0.277) - 0.234, b: radial * (0.312 + 0.199) - 0.312},
            a: 1.0
        }
    }

    fn to_wheel(&self) -> (f32, f32, f32) {
        (
            (self.oklab.a + 0.234) / (0.234 + 0.277),
            (self.oklab.b + 0.312) / (0.312 + 0.199),
            self.oklab.l
        )
    }

    fn gradient(&self, other: [f32; 4], start_index: u8, end_index: u8, len: u8) -> Vec<Self> where Self: Sized {
        todo!();
        vec![Self {
            oklab: Oklab { l: 0.0, a: 0.0, b: 0.0 },
            a: 1.0
        }; len as usize]
    }
}