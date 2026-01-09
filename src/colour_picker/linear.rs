use macroquad::prelude::{Rect, Vec2, draw_rectangle, draw_triangle, Color, draw_circle_lines};

use crate::{colour::Col, colour_picker::picker::Circular};
use super::{ColSelection, ColPicker, PickerEnum, PickerSelection};


pub struct Linear {
    pub offset: [f32; 2],
    pub height: f32,
    pub width: f32,
    pub padding: f32,
    pub selected: Option<Col>,
    pub coltype: ColSelection,
    cache: Option<[f32; 4]>
}

impl Linear {
    pub fn new(height: f32, width: f32, offset: [f32;2], padding: f32, coltype: ColSelection) -> Self {
        Self {
            offset,
            height,
            width,
            padding,
            selected: None,
            coltype,
            cache: None
        }
    }

    pub fn with_col(height: f32, width: f32, offset: [f32;2], padding: f32, coltype: ColSelection, selected: Option<Col>) -> Self {
        Self {
            offset,
            height,
            width,
            padding,
            selected,
            coltype,
            cache: None
        }
    }
}

impl ColPicker for Linear {
    fn bounding_box(&self) -> Rect {
        let height = self.height;
        let width = height + self.padding + self.width;

        Rect::new(self.offset[0], self.offset[1], width, height)
    }
    
    fn detect(&mut self, mouse: Vec2, first_mouse_down: Vec2) {
        let offset = Vec2::new(self.offset[0], self.offset[1]);
        let mouse = mouse - offset;
        let slider_rect = Rect::new(self.height + self.padding, 0.0, self.width, self.height);
        let picker_rect = Rect::new(0.0, 0.0, self.height, self.height);

        if slider_rect.contains(first_mouse_down - offset) {
            self.cache = None;
            let scalar = (1.0 - mouse.y / self.height).clamp(0.0, 1.0);
            match &self.selected {
                Some(col) => {
                    let (circular, radial, _scalar) = col.to_wheel();
                    self.selected = Some(self.coltype.col_from_wheel(circular, radial, scalar))
                },
                None => self.selected = Some(self.coltype.col_from_wheel(self.coltype.default_cirular(), self.coltype.default_radial(), scalar))
            }
        } else if picker_rect.contains(first_mouse_down - offset) {
            self.cache = None;
            let radial = (1.0 - mouse.y / self.height).clamp(0.0, 1.0);
            let circular = (mouse.x / self.height).clamp(0.0, 1.0);
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

        let iheight = self.height as i16;
        let div_1_height = 1.0 / self.height;

        for x in 0..iheight {
            for y in 0..iheight {
                let x = x as f32;
                let y = y as f32;

                let radial = 1.0 - y * div_1_height;
                let circular = x * div_1_height;

                draw_rectangle(
                    x + self.offset[0],
                    y + self.offset[1],
                    1.0,
                    1.0,
                    self.coltype.col_from_wheel(circular, radial, scalar).to_macroquad_col()
                );
            }
        }

        let x = self.offset[0] + self.height + self.padding;

        for value in 0..iheight {
            let value = value as f32;
            let y = value + self.offset[1];
            draw_rectangle(x, y, self.width, 1.0, self.coltype.col_from_wheel(circular, radial, 1.0 - value * div_1_height).to_macroquad_col());
        }

        if let Some((circular, radial, scalar)) = wheel_selected {
            let y = self.height * (1.0 - scalar) + self.offset[1];
            draw_triangle(
                Vec2::new(x, y),
                Vec2::new(x - self.padding/2.0, y - self.padding/3.0),
                Vec2::new(x - self.padding/2.0, y + self.padding/3.0),
                Color::from_hex(0xFFFFFF)
                );

            let x = self.offset[0] + circular * self.height;
            let y = self.offset[1] + (1.0 - radial) * self.height;
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
        self.selected = self.selected.as_ref().map(|d| coltype.col_from_rgba_arr(d.to_rgba()));
        self.coltype = coltype;
    }

    fn transfer_picker(self, pickertype: PickerSelection) -> PickerEnum {
        match pickertype {
            PickerSelection::Circular => PickerEnum::Circular(Circular::with_col(self.height / 2.0, self.width, self.offset, self.padding, self.coltype, self.selected)),
            PickerSelection::Linear => PickerEnum::Linear(self),
        }
    }
}