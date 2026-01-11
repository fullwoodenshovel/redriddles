use super::*;

pub struct ChangeStatus<const INDEX: u8> {
    name: String,
    change_to: u8,
    rect: Rect
}

impl<const INDEX: u8> NewInOut for ChangeStatus<INDEX> {
    type InType = (String, f32, u8);
    type OutType = f32;
    fn new((name, x_offset, change_to): Self::InType, _handler: &mut GenHandler) -> (Self::OutType, Self) {
        let text_width = measure_text(&name, None, 18, 1.0).width;
        (
            text_width + 16.0,
            Self {
                name,
                change_to,
                rect: Rect::new(x_offset, INDEX as f32 * 42.0 + 6.0, text_width + 16.0, 28.0)
            }
        )
    }
}

impl<const INDEX: u8> Node for ChangeStatus<INDEX> {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        let status = ctx.store.value::<Status<INDEX>>();
        if helpers::ui_button(
            self.rect,
            &self.name,
            ctx.user_inputs.hover_test(node),
            ctx.user_inputs.left_let_go,
            if status == self.change_to { ENABLEDCOL } else { DISABLEDCOL },
            if status == self.change_to { ENABLEDHOVERCOL } else { DISABLEDHOVERCOL }
        ) {
            ctx.store.set::<Status<INDEX>>(self.change_to);
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