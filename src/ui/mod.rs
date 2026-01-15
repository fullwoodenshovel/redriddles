pub use macroquad::miniquad::window::screen_size;
pub use macroquad::prelude::*;

pub const DISABLEDCOL: Color = LIGHTGRAY;
pub const DISABLEDHOVERCOL: Color = Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 };
pub const ENABLEDCOL: Color = Color { r: 0.65, g: 0.8, b: 0.65, a: 1.0 };
pub const ENABLEDHOVERCOL: Color = Color { r: 0.60, g: 0.75, b: 0.60, a: 1.0 };

use super::colour_picker;
use super::colour;
use super::helpers;
pub use super::transform::*;
pub use super::colour::*;
pub use super::node::*;
pub use super::helpers::*;
pub mod main;
use tuple_deref::tuple_deref;