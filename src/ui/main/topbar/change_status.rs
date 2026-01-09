use super::*;

pub struct ChangeStatus<const INDEX: u8> {
    name: String,
    rect: Rect,
    change_to: u8
}

impl<const INDEX: u8> NewNoOut for ChangeStatus<INDEX> {
    type InType = (String, Rect, u8);
    fn new((name, rect, change_to): Self::InType, _handler: &mut GenHandler) -> Self {
        Self {
            name,
            rect,
            change_to
        }
    }
}

impl<const INDEX: u8> Node for ChangeStatus<INDEX> {
    fn update(&mut self, ctx: &mut AppContextHandler, _node: &NodeStore) {
        let status = ctx.store.value::<Status<INDEX>>();
        if helpers::ui_button(
            self.rect,
            &self.name,
            if ctx.store.value::<HoverPossible>() {Some(ctx.user_inputs.mouse)} else {None},
            ctx.user_inputs.left_let_go,
            if status == self.change_to { Color { r: 0.65, g: 0.8, b: 0.65, a: 1.0 } } else { LIGHTGRAY },
            if status == self.change_to { Color { r: 0.60, g: 0.75, b: 0.60, a: 1.0 } } else { Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 } }
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