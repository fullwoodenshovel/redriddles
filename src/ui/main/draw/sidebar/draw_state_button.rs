use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum DrawState {
    Line,
    Fill,
    Picker,
    Draw
}

pub struct DrawStateButton {
    name: &'static str,
    pub rect: Rect,
    new_state: DrawState
}

impl NewNoOut for DrawStateButton {
    type InType = (&'static str, Rect, DrawState);
    fn new((name, rect, new_state): Self::InType, _handler: &mut GenHandler) -> Self {
        Self {
            name,
            rect,
            new_state
        }
    }
}

impl Node for DrawStateButton {
    fn update(&mut self, ctx: &mut AppContextHandler, node: &NodeStore) {
        let hover_possible = ctx.store.value::<HoverPossible>();
        let lasttouch = ctx.user_inputs.last_touch_test(node);
        let active = *ctx.store.get::<DrawState>() == self.new_state;

        helpers::ui_button(
            self.rect,
            self.name,
            if hover_possible {Some(ctx.user_inputs.mouse)} else {None},
            ctx.user_inputs.left_let_go,
            if active { Color { r: 0.65, g: 0.8, b: 0.65, a: 1.0 } } else { LIGHTGRAY },
            if active { Color { r: 0.60, g: 0.75, b: 0.60, a: 1.0 } } else { Color { r: 0.65, g: 0.65, b: 0.65, a: 1.0 } }
        );

        if lasttouch && hover_possible && ctx.user_inputs.left_let_go {
            if active {
                ctx.store.overwrite(DrawState::Draw);
            } else {
                ctx.store.overwrite(self.new_state);
            }
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