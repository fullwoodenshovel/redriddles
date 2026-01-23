use macroquad::prelude::*;

use crate::{colour::Col, colour_picker::{SurfaceCache, picker::Circular}};
use super::{ColSelection, ColPicker, PickerEnum, PickerSelection};


pub struct Linear {
    pub offset: [f32; 2],
    pub height: f32,
    pub width: f32,
    pub padding: f32,
    pub selected: Option<Col>,
    pub coltype: ColSelection,
    cached_col: Option<Option<[f32; 4]>>,
    surface_cache: SurfaceCache
}

impl Linear {
    pub fn new(height: f32, width: f32, offset: [f32;2], padding: f32, coltype: ColSelection) -> Self {
        Self::with_col(height, width, offset, padding, coltype, None)
    }

    pub fn with_col(height: f32, width: f32, offset: [f32;2], padding: f32, coltype: ColSelection, selected: Option<[f32; 4]>) -> Self {
        let mut result = Self {
            offset,
            height,
            width,
            padding,
            selected: selected.map(|d| coltype.col_from_rgba_arr(d)),
            coltype,
            cached_col: Some(selected),
            surface_cache: SurfaceCache::new(Rect::default())
        };

        let rect = result.bounding_box();
        result.surface_cache = SurfaceCache::new(rect);
        result
    }
}

impl ColPicker for Linear {
    fn bounding_box(&self) -> Rect {
        let height = self.height;
        let width = height + self.padding + self.width;

        Rect::new(self.offset[0], self.offset[1], width, height)
    }
    
    fn detect(&mut self, mouse: Vec2, first_mouse_down: Vec2) {
        let offset = vec2(self.offset[0], self.offset[1]);
        let mouse = mouse - offset;
        let slider_rect = Rect::new(self.height + self.padding, 0.0, self.width, self.height);
        let picker_rect = Rect::new(0.0, 0.0, self.height, self.height);

        if slider_rect.contains(first_mouse_down - offset) {
            self.cached_col = None;
            self.surface_cache.invalidate();
            let scalar = (1.0 - mouse.y / self.height).clamp(0.0, 1.0);
            match &self.selected {
                Some(col) => {
                    let (circular, radial, _scalar) = col.to_wheel();
                    self.selected = Some(self.coltype.col_from_wheel(circular, radial, scalar))
                },
                None => self.selected = Some(self.coltype.col_from_wheel(self.coltype.default_cirular(), self.coltype.default_radial(), scalar))
            }
        } else if picker_rect.contains(first_mouse_down - offset) {
            self.cached_col = None;
            self.surface_cache.invalidate();
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

    fn draw(&mut self) {
        let wheel_selected = if let Some(_guard) = self.surface_cache.redraw() {
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
                        x,
                        y,
                        1.0,
                        1.0,
                        self.coltype.col_from_wheel(circular, radial, scalar).to_macroquad_col()
                    );
                }
            }
            
            let x = self.height + self.padding;
            
            for value in 0..iheight {
                let value = value as f32;
                let y = value;
                draw_rectangle(x, y, self.width, 1.0, self.coltype.col_from_wheel(circular, radial, 1.0 - value * div_1_height).to_macroquad_col());
            }
            wheel_selected
        } else {
            self.selected.as_ref().map(|col| col.to_wheel())
        };
        if let Some((circular, radial, scalar)) = wheel_selected {
            let x = self.height + self.padding + self.offset[0];
            let y = self.height * (1.0 - scalar) + self.offset[1];
            draw_triangle(
                vec2(x, y),
                vec2(x - self.padding/2.0, y - self.padding/3.0),
                vec2(x - self.padding/2.0, y + self.padding/3.0),
                Color::from_hex(0xFFFFFF)
                );
        
            let x = circular * self.height + self.offset[0];
            let y = (1.0 - radial) * self.height + self.offset[1];
            draw_circle_lines(x, y, 4.0, 3.0, Color::from_hex(0xFFFFFF));
        }
    }
    
    fn get_col_rgba(&mut self) -> Option<[f32; 4]> {
        match self.cached_col {
            Some(cached_col) => cached_col,
            None => {
                let result = self.selected.as_ref().map(|d| d.to_rgba());
                self.cached_col = Some(result);
                result
            }
        }
    }

    fn set_col(&mut self, col: Option<[f32; 4]>) {
        self.cached_col = Some(col);
        self.selected = col.map(|d| self.coltype.col_from_rgba_arr(d));
        self.surface_cache.invalidate();
    }
    
    fn transfer_col(&mut self, coltype: ColSelection) {
        self.selected = self.selected.as_ref().map(|d| coltype.col_from_rgba_arr(d.to_rgba()));
        self.coltype = coltype;
        self.surface_cache.invalidate();
    }
    
    fn transfer_picker(mut self, pickertype: PickerSelection) -> PickerEnum {
        match pickertype {
            PickerSelection::Circular => PickerEnum::Circular(Circular::with_col(self.height / 2.0, self.width, self.offset, self.padding, self.coltype, self.get_col_rgba())),
            PickerSelection::Linear => PickerEnum::Linear(self),
        }
    }
}