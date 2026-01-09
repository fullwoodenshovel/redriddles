use super::*;
use topbar::Topbar;
use sidebar::Sidebar;
use draw_section::DrawSection;

pub struct Draw {
    node: WeakNode // 0 is Topbar, 1 is Sidebar, 2 is DrawSection
}

impl New for Draw {
    type InType = ();
    fn new(_: Self::InType, node: WeakNode) -> Self {
        let binding = node.upgrade().unwrap();
        let mut node_ref = binding.borrow_mut();

        node_ref.push_child::<Topbar>(());
        node_ref.push_child::<Sidebar>(None);
        node_ref.push_child::<DrawSection>(());

        Self {
            node
        }
    }
}

impl Node for Draw {
    fn update(&mut self, user_inputs: &UserInputs) {
        clear_background(WHITE);
        self.node.upgrade().unwrap().borrow_mut().update_children(user_inputs);
    }

    fn hit_detect(&self, pos: Vec2) -> Vec<WeakNode> {
        recur_hit_detect(&self.node, pos)
    }
}