use super::*;
use colour_picker::Picker;

pub struct Eraser {
    pub rect: Rect
}

impl New for Eraser {
    fn new(_handler: &mut GenHandler) -> Self {
        Self {
            rect: Rect::new(10.0, 482.0, 133.0, 28.0),
        }
    }
}

impl Node for Eraser {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        let hovered = ctx.user_inputs.hover_test(node);
        let active = ctx.store.get_mut::<Picker>().get_col_rgba().is_none();
            
        if helpers::ui_button(
            self.rect,
            "Eraser",
            hovered,
            ctx.user_inputs.left_let_go,
            if active { ENABLEDCOL } else { DISABLEDCOL },
            if active { ENABLEDHOVERCOL } else { DISABLEDHOVERCOL }
        ) {
            ctx.store.get_mut::<Picker>().set_col(None);
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