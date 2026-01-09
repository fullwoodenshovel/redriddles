use super::*;

pub struct Settings;

impl NewNoOut for Settings {
    type InType = ();
    fn new(_data: Self::InType, _handler: &mut GenHandler) -> Self {
        Self // todo!()
    }
}

impl Node for Settings {
    fn update(&mut self, _user_inputs: &UserInputs, _node: &NodeStore) {
        todo!()
    }
    
    fn hit_detect(&mut self, _pos: Vec2, _node: &NodeStore) -> Vec<WeakNode> {
        todo!()
    }
}