use super::*;
use draw::Draw;
use settings::Settings;
use export::Export;

pub struct Main {
    status: usize,
    node: WeakNode // 0 is draw, 1 is settings, 2 is export
}

impl New for Main {
    type InType = ();
    fn new(_: Self::InType, node: WeakNode) -> Self {
        let binding = node.upgrade().unwrap();
        let mut node_ref = binding.borrow_mut();
        node_ref.push_child::<Draw>(());
        node_ref.push_child::<Settings>(());
        node_ref.push_child::<Export>(());
        Self { status: 0, node }
    }
}

impl Node for Main {
    fn update(&mut self, user_inputs: &UserInputs) {
        self.node.upgrade().unwrap().borrow().children[self.status].borrow_mut().get_self_dyn_mut().update(user_inputs);
    }

    fn hit_detect(&self, pos: Vec2) -> Vec<WeakNode> {
        let mut result = self.node.upgrade().unwrap().borrow().children[self.status].borrow().get_self_dyn().hit_detect(pos);
        result.push(self.node.clone());
        result
    }
}
