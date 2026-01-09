use super::*;

#[derive(Default)]
pub struct Export;

impl New for Export {
    type InType = ();
    fn new(_data: Self::InType, _node: WeakNode) -> Self {
        Self // todo!()
    }
}

impl Node for Export {
    fn update(&mut self, _user_inputs: &UserInputs) {
        todo!()
    }
    
    fn hit_detect(&self, _pos: Vec2) -> Vec<WeakNode> {
        todo!()
    }
}