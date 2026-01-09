use crate::ui::main::topbar::HoverPossible;

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
        let hover_possible = ctx.store.value::<HoverPossible>();
        let lasttouch = ctx.user_inputs.last_touch_test(node);
        ctx.store.overwrite(LastTouch(lasttouch));
        ctx.store.overwrite(PrevTouch(ctx.user_inputs.prev_last_touch_test(node)));

        if let Some(col) = ctx.store.get_mut::<Picker>().get_col_rgba() {
            
            helpers::ui_button(
                self.rect,
                "Save colour",
                if hover_possible {Some(ctx.user_inputs.mouse)} else {None},
                ctx.user_inputs.left_let_go,
                if lasttouch { Color { r: 0.65, g: 0.8, b: 0.65, a: 1.0 } } else { LIGHTGRAY },
                if lasttouch { Color { r: 0.60, g: 0.75, b: 0.60, a: 1.0 } } else { Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 } }
            );

            draw_rectangle(115.0, 380.0, 28.0, 28.0, LIGHTGRAY);
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