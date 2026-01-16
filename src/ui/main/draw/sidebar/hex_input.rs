use super::*;
use colour_picker::Picker;

pub struct HexInput {
    pub rect: Rect,
    text: String,
    pub active: bool,
}

impl New for HexInput {
    fn new(_handler: &mut GenHandler) -> Self {
        Self {
            rect: Rect::new(10.0, 414.0, 133.0, 28.0),
            text: String::from("#"),
            active: false,
        }
    }
}

impl Node for HexInput {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        if ui_button(
            self.rect,
            &self.text,
            if self.active {ENABLEDCOL} else {DISABLEDCOL},
            if self.active {ENABLEDCOL} else {DISABLEDHOVERCOL},
            node,
            ctx
        ) {
            self.active = true;
            ctx.store.get_mut::<Picker>().set_col(None);
            self.text = "#".to_string();
            while get_char_pressed().is_some() {}
        }

        if self.active && !ctx.user_inputs.last_touch_test(node) {
            self.active = false
        }

        // input
        if self.active {
            ctx.user_inputs.disable_shortcuts();
            while let Some(c) = get_char_pressed() {
                ctx.store.get_mut::<Picker>().set_col(None);
                if self.text.len() < 7 && c.is_ascii_hexdigit() {
                    self.text.push(c.to_ascii_uppercase());
                    if self.text.len() == 7 {
                        ctx.store.get_mut::<Picker>().set_col(Some(ColSelection::Rgba.col_from_hex_string(&self.text[1..]).to_rgba()));
                        self.active = false;
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
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, _store: &mut Store) -> Vec<WeakNode> {
        if self.rect.contains(pos) {
            vec![node.get_weak()]
        } else {
            vec![]
        }
    }
}