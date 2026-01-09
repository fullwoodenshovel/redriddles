use super::*;
use draw::Draw;
use settings::Settings;
use export::Export;

pub struct Main {
    status: usize
}

impl NewNoOut for Main {
    type InType = ();
    fn new(_: Self::InType, handler: &mut GenHandler) -> Self { // 0 is draw, 1 is settings, 2 is export
        handler.push_child::<Draw>(());
        handler.push_child::<Settings>(());
        handler.push_child::<Export>(());
        Self { status: 0 }
    }
}

impl Node for Main {
    fn update(&mut self, user_inputs: &UserInputs, node: &NodeStore) {
        node.get_children()[self.status].update(user_inputs);
    }

    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore) -> Vec<WeakNode> {
        let mut result = node.get_children()[self.status].hit_detect(pos);
        result.push(node.get_weak());
        result
    }
}
