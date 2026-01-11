use super::*;
use colour_picker::Picker;

pub struct HexInput {
    pub rect: Rect,
    text: String,
    pub active: bool,
    default_col: Color,
    hover_col: Color,
    active_col: Color,
}

impl New for HexInput {
    fn new(_handler: &mut GenHandler) -> Self {
        Self {
            rect: Rect::new(10.0, 414.0, 133.0, 28.0),
            text: String::from("#"),
            active: false,
            default_col: DISABLEDCOL,
            hover_col: DISABLEDHOVERCOL,
            active_col: ENABLEDCOL
        }
    }
}

impl Node for HexInput {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        let active_possible = ctx.user_inputs.last_touch_test(node);
        let let_go = ctx.user_inputs.left_let_go;

        let hovered = ctx.user_inputs.hover_test(node);

        if hovered && let_go {
            self.active = true;
            ctx.store.get_mut::<Picker>().set_col(None);
            self.text = "#".to_string();
            while get_char_pressed().is_some() {}
        }

        if !active_possible {
            self.active = false
        }

        
        // input
        if self.active {
            while let Some(c) = get_char_pressed() {
                ctx.store.get_mut::<Picker>().set_col(None);
                if self.text.len() < 7 && c.is_ascii_hexdigit() {
                    self.text.push(c.to_ascii_uppercase());
                    if self.text.len() == 7 {
                        ctx.store.get_mut::<Picker>().set_col(Some(ColSelection::Rgba.col_from_hex_string(&self.text[1..]).to_rgba()));
                    }
                }
            }

            if is_key_pressed(KeyCode::Backspace) {
                self.text.pop();
                if self.text.is_empty() {
                    self.text.push('#');
                }
            }
        } else if let Some(col) = ctx.store.get_mut::<Picker>().get_col_rgba() {
            self.text = ColSelection::format_rgba(col);
        } else {
            self.text = "NA".to_string();
        }

        // draw box
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            if self.active { self.active_col } else if hovered { self.hover_col } else { self.default_col },
        );

        draw_text(
            &self.text,
            self.rect.x + 6.0,
            self.rect.y + 22.0,
            22.0,
            BLACK,
        );
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, _store: &mut Store) -> Vec<WeakNode> {
        if self.rect.contains(pos) {
            vec![node.get_weak()]
        } else {
            vec![]
        }
    }
}