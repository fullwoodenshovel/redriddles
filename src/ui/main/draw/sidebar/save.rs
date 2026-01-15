use super::*;
use colour_picker::Picker;

pub struct Save {
    pub rect: Rect,
}

#[tuple_deref]
pub struct LastTouch(bool);

#[tuple_deref]
pub struct PrevTouch(bool);

impl New for Save {
    fn new(handler: &mut GenHandler) -> Self {
        handler.push_data(LastTouch(false));
        handler.push_data(PrevTouch(false));

        Self {
            rect: Rect::new(10.0, 380.0, 100.0, 28.0)
        }
    }
}

impl Node for Save {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        let hovered = ctx.user_inputs.hover_test(node);
        let lasttouch = ctx.user_inputs.last_touch_test(node);
        ctx.store.overwrite(LastTouch(lasttouch));
        ctx.store.overwrite(PrevTouch(ctx.user_inputs.prev_last_touch_test(node)));

        if let Some(col) = ctx.store.get_mut::<Picker>().get_col_rgba() {
            
            raw_ui_button(
                self.rect,
                "Save colour",
                hovered,
                false,
                if lasttouch { ENABLEDCOL } else { DISABLEDCOL },
                if lasttouch { ENABLEDHOVERCOL } else { DISABLEDHOVERCOL },
            );

            draw_rectangle(115.0, 380.0, 28.0, 28.0, DISABLEDCOL);
            draw_rectangle(120.0, 385.0, 18.0, 18.0, arr_to_macroquad(col));
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