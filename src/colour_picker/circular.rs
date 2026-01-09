use macroquad::prelude::{Rect, Vec2, draw_rectangle, draw_triangle, Color, draw_circle_lines};

use crate::{colour::Col, colour_picker::linear::Linear};
use super::{ColSelection, ColPicker, PickerEnum, PickerSelection};


pub struct Circular {
    pub offset: [f32; 2],
    pub radius: f32,
    pub width: f32,
    pub padding: f32,
    pub selected: Option<Col>,
    pub coltype: ColSelection,
    cache: Option<[f32; 4]>
}

impl Circular {
    pub fn new(radius: f32, width: f32, offset: [f32;2], padding: f32, coltype: ColSelection) -> Self {
        Self {
            offset,
            radius,
            width,
            padding,
            selected: None,
            coltype,
            cache: None
        }
    }

    pub fn with_col(radius: f32, width: f32, offset: [f32;2], padding: f32, coltype: ColSelection, selected: Option<Col>) -> Self {
        Self {
            offset,
            radius,
            width,
            padding,
            selected,
            coltype,
            cache: None
        }
    }
}

impl ColPicker for Circular {
    fn bounding_box(&self) -> Rect {
        let height = self.radius * 2.0;
        let width = height + self.padding + self.width;

        Rect::new(self.offset[0], self.offset[1], width, height)
    }
    
    fn detect(&mut self, mouse: Vec2, first_mouse_down: Vec2) {
        let offset = Vec2::new(self.offset[0], self.offset[1]);
        let mouse = mouse - offset;
        let circle_mouse = mouse - self.radius;
        let height = self.radius * 2.0;
        let radial = circle_mouse.length() / self.radius;
        let slider_rect = Rect::new(height + self.padding, 0.0, self.width, height);

        if slider_rect.contains(first_mouse_down - offset) {
            self.cache = None;
            let scalar = (1.0 - mouse.y / height).clamp(0.0, 1.0);
            match &self.selected {
                Some(col) => {
                    let (circular, radial, _scalar) = col.to_wheel();
                    self.selected = Some(self.coltype.col_from_wheel(circular, radial, scalar))
                },
                None => self.selected = Some(self.coltype.col_from_wheel(self.coltype.default_cirular(), self.coltype.default_radial(), scalar))
            }
        } else if (first_mouse_down - offset - self.radius).length() / self.radius <= 1.0 {
            self.cache = None;
            let radial = radial.clamp(0.0, 1.0);
            let circular = circle_mouse.y.atan2(-circle_mouse.x) / (std::f32::consts::TAU) + 0.5;
            match &self.selected {
                Some(col) => {
                    let (_circular, _radial, scalar) = col.to_wheel();
                    self.selected = Some(self.coltype.col_from_wheel(circular, radial, scalar))
                },
                None => self.selected = Some(self.coltype.col_from_wheel(circular, radial, self.coltype.default_scalar()))
            }
        }
    }

    fn draw(&self) {
        let wheel_selected = self.selected.as_ref().map(|col| col.to_wheel());
        let (circular, radial, scalar) = if let Some((circular, radial, scalar)) = wheel_selected {
            (circular, radial, scalar)
        } else {
            (self.coltype.default_cirular(), self.coltype.default_radial(), self.coltype.default_scalar())
        };

        let center = [self.offset[0] + self.radius, self.offset[1] + self.radius];
        let radius_squared = self.radius * self.radius;
        let iradius = self.radius as i16;

        for x in -iradius..iradius {
            for y in -iradius..iradius {
                let x = x as f32;
                let y = y as f32;
                let distance_squared = x*x + y*y;
                if distance_squared >= radius_squared {
                    continue;
                }

                let radial = (distance_squared / radius_squared).sqrt();
                let circular = y.atan2(-x) / (std::f32::consts::TAU) + 0.5;

                draw_rectangle(
                    x + center[0],
                    y + center[1],
                    1.0,
                    1.0,
                    self.coltype.col_from_wheel(circular, radial, scalar).to_macroquad_col()
                );
            }
        }

        let height = self.radius as u16 * 2;
        let max_value = height as f32;
        let x = self.offset[0] + max_value + self.padding;

        for value in 0..height {
            let value = value as f32;
            let y = value + self.offset[1];
            draw_rectangle(x, y, self.width, 1.0, self.coltype.col_from_wheel(circular, radial, 1.0 - value / max_value).to_macroquad_col());
        }

        if let Some((circular, radial, scalar)) = wheel_selected {
            let height = height as f32;
            let y = height * (1.0 - scalar) + self.offset[1];
            draw_triangle(
                Vec2::new(x, y),
                Vec2::new(x - self.padding/2.0, y - self.padding/3.0),
                Vec2::new(x - self.padding/2.0, y + self.padding/3.0),
                Color::from_hex(0xFFFFFF)
                );

            let x = center[0] + f32::cos(circular * std::f32::consts::TAU) * radial * self.radius;
            let y = center[1] - f32::sin(circular * std::f32::consts::TAU) * radial * self.radius;
            draw_circle_lines(x, y, 4.0, 3.0, Color::from_hex(0xFFFFFF));
        }
    }

    fn get_col_rgba(&mut self) -> Option<[f32; 4]> {
        if self.cache.is_none() { self.cache = self.selected.as_ref().map(|d| d.to_rgba()) }
        self.cache
    }

    fn set_col(&mut self, col: Option<[f32; 4]>) {
        self.cache = col;
        self.selected = col.map(|d| self.coltype.col_from_rgba_arr(d));
    }

    fn transfer_col(&mut self, coltype: ColSelection) {
        if coltype == self.coltype { return; };
        self.selected = self.selected.as_ref().map(|d| coltype.col_from_rgba_arr(d.to_rgba()));
        self.coltype = coltype;
    }

    fn transfer_picker(self, pickertype: PickerSelection) -> PickerEnum {
        match pickertype {
            PickerSelection::Circular => PickerEnum::Circular(self),
            PickerSelection::Linear => PickerEnum::Linear(Linear::with_col(self.radius * 2.0, self.width, self.offset, self.padding, self.coltype, self.selected)),
        }
    }
}