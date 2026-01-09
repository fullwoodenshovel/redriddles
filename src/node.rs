use crate::ui::ColSelection;
use crate::ui::main::DrawState;

#[cfg(feature = "debug")]
use super::debug::RefCell;
#[cfg(not(feature = "debug"))]
use std::cell::RefCell;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use bimap::BiMap;

use macroquad::prelude::*;

use super::helpers::*;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub enum ShortcutInstruction {
    None,
    ChangeDrawState(DrawState),
    Eraser,
    SaveCol,
    ToggleGrid,
    ChangePickerType(ColSelection),
}

pub struct Shortcuts {
    shortcuts: BiMap<Vec<KeyCode>, ShortcutInstruction>
}

impl Default for Shortcuts {
    fn default() -> Self {
        Self {
            shortcuts: vec![
                (vec![KeyCode::S], ShortcutInstruction::SaveCol),
                (vec![KeyCode::E], ShortcutInstruction::Eraser),
                (vec![KeyCode::H], ShortcutInstruction::ChangePickerType(ColSelection::Hsva)),
                (vec![KeyCode::R], ShortcutInstruction::ChangePickerType(ColSelection::Rgba)),
                (vec![KeyCode::O], ShortcutInstruction::ChangePickerType(ColSelection::OkLab)),
                (vec![KeyCode::F], ShortcutInstruction::ChangeDrawState(DrawState::Fill)),
                (vec![KeyCode::L], ShortcutInstruction::ChangeDrawState(DrawState::Line)),
                (vec![KeyCode::D], ShortcutInstruction::ChangeDrawState(DrawState::Draw)),
                (vec![KeyCode::P], ShortcutInstruction::ChangeDrawState(DrawState::Picker)),
                (vec![KeyCode::G], ShortcutInstruction::ToggleGrid),
            ].into_iter().map(|(mut key, value)| {
                Self::sort_key(&mut key);
                (key, value)
            }).collect()
        }
    }
}

impl Shortcuts {
    pub fn insert_no_overwrite(&mut self, mut key: Vec<KeyCode>, value: ShortcutInstruction) -> Result<(), (Vec<KeyCode>, ShortcutInstruction)> {
        Self::sort_key(&mut key);
        self.shortcuts.insert_no_overwrite(key, value)
    }

    pub fn insert(&mut self, mut key: Vec<KeyCode>, value: ShortcutInstruction) {
        Self::sort_key(&mut key);
        self.shortcuts.insert(key, value);
    }

    pub fn get_shortcuts(&self) -> Vec<(&Vec<KeyCode>, ShortcutInstruction)> {
        self.shortcuts.iter().map(|(k, &v)| (k, v)).collect()
    }

    pub fn get_output(&self, shortcut: &mut Vec<KeyCode>) -> ShortcutInstruction {
        Self::sort_key(shortcut);
        *self.shortcuts.get_by_left(shortcut).unwrap_or(&ShortcutInstruction::None)
    }

    pub fn get_shortcut(&self, output: &ShortcutInstruction) -> Option<&Vec<KeyCode>> {
        self.shortcuts.get_by_right(output)
    }

    fn sort_key(key: &mut [KeyCode]) {
        key.sort_by(|&a, &b| (a as u16).cmp(&(b as u16)));
    }
}

pub struct UserInputs {
    pub hoverhold_focus: Vec<WeakNode>,
    pub prev_hoverhold_focus: Vec<WeakNode>,
    pub hoverhold_mouse: Vec2,
    pub hover_focus: Vec<WeakNode>,
    pub prev_hover_focus: Vec<WeakNode>,
    pub lasttouch_focus: Vec<WeakNode>,
    pub lasttouch_mouse: Vec2,
    pub prev_lasttouch_focus: Vec<WeakNode>,
    pub prev_lasttouch_mouse: Vec2,
    pub mouse: Vec2,
    pub prev_mouse: Vec2,
    pub left_mouse_pressed: bool,
    pub right_mouse_pressed: bool,
    pub left_mouse_down: bool,
    pub right_mouse_down: bool,
    pub left_let_go: bool,
    pub right_let_go: bool,
    pub pressed_shortcut: ShortcutInstruction,
    pub held_shortcut: ShortcutInstruction,
    pub shortcuts: Shortcuts,
    origin: StrongNode,
}


impl UserInputs {
    pub fn new(origin: &Origin) -> Self {
        Self {
            hoverhold_focus: vec![],
            prev_hoverhold_focus: vec![],
            hoverhold_mouse: Vec2::splat(0.0),
            hover_focus: vec![],
            prev_hover_focus: vec![],
            lasttouch_focus: vec![],
            lasttouch_mouse: Vec2::splat(0.0),
            prev_lasttouch_focus: vec![],
            prev_lasttouch_mouse: Vec2::splat(0.0),
            mouse: Vec2::splat(0.0),
            prev_mouse: Vec2::splat(0.0),
            left_mouse_pressed: false,
            right_mouse_pressed: false,
            left_mouse_down: false,
            right_mouse_down: false,
            left_let_go: false,
            right_let_go: false,
            pressed_shortcut: ShortcutInstruction::None,
            held_shortcut: ShortcutInstruction::None,
            shortcuts: Shortcuts::default(),
            origin: origin.node.clone(),
        }
    }

    pub fn update(&mut self, store: &mut Store) {
        self.prev_mouse = self.mouse;
        self.mouse = mouse_vec();
        self.left_mouse_pressed = is_mouse_button_pressed(MouseButton::Left);
        self.right_mouse_pressed = is_mouse_button_pressed(MouseButton::Right);
        self.left_mouse_down = is_mouse_button_down(MouseButton::Left);
        self.right_mouse_down = is_mouse_button_down(MouseButton::Right);
        self.left_let_go = is_mouse_button_released(MouseButton::Left);
        self.right_let_go = is_mouse_button_released(MouseButton::Right);

        self.prev_hover_focus = std::mem::take(&mut self.hover_focus);
        self.hover_focus = Handler::new(&self.origin).hit_detect(self.mouse, store);

        let new = self.shortcuts.get_output(&mut get_keys_down().into_iter().collect());
        if new != self.held_shortcut {
            self.pressed_shortcut = new;
        } else {
            self.pressed_shortcut = ShortcutInstruction::None;
        }
        self.held_shortcut = new;

        if self.left_mouse_pressed || !self.left_mouse_down {
            self.prev_hoverhold_focus = std::mem::take(&mut self.hoverhold_focus);
            self.hoverhold_focus = self.hover_focus.clone();
            self.hoverhold_mouse = self.mouse;
            if self.left_mouse_pressed {
                self.prev_lasttouch_focus = std::mem::take(&mut self.lasttouch_focus);
                self.prev_lasttouch_mouse = self.lasttouch_mouse;
                self.lasttouch_focus = self.hover_focus.clone();
                self.lasttouch_mouse = self.mouse;
            }
        }
    }

    pub fn hoverhold_test(&self, node: &NodeStore) -> bool {
        node.contains_self(&self.hoverhold_focus)
    }

    pub fn prev_hoverhold_test(&self, node: &NodeStore) -> bool {
        node.contains_self(&self.prev_hoverhold_focus)
    }

    pub fn hover_test(&self, node: &NodeStore) -> bool {
        node.contains_self(&self.hover_focus)
    }

    pub fn prev_hover_test(&self, node: &NodeStore) -> bool {
        node.contains_self(&self.prev_hover_focus)
    }

    pub fn last_touch_test(&self, node: &NodeStore) -> bool {
        node.contains_self(&self.lasttouch_focus)
    }

    pub fn prev_last_touch_test(&self, node: &NodeStore) -> bool {
        node.contains_self(&self.prev_lasttouch_focus)
    }

    pub fn instruction_pressed(&self, instruction: ShortcutInstruction) -> bool {
        self.pressed_shortcut == instruction
    }

    pub fn instruction_active(&self, instruction: ShortcutInstruction) -> bool {
        self.held_shortcut == instruction
    }
}

struct AppContext {
    user_inputs: UserInputs,
    store: Store
}

impl AppContext {
    fn get_handler(&'_ mut self) -> AppContextHandler<'_> {
        AppContextHandler { user_inputs: &self.user_inputs, store: &mut self.store }
    }
}

pub struct AppContextHandler<'a> {
    pub user_inputs: &'a UserInputs,
    pub store: &'a mut Store
}

pub struct Store {
    store: HashMap<TypeId, Box<dyn Any>>
}

impl Store {
    fn new() -> Self {
        Self {
            store: HashMap::new()
        }
    }

    pub fn get<T: Any>(&self) -> &T {
        self.store
            .get(&TypeId::of::<T>())
            .unwrap_or_else(|| panic!("Data expected from AppContextGenHandler of a type that hasn't been entered.\nT = {}", std::any::type_name::<T>()))
            .downcast_ref()
            .unwrap()
    }

    pub fn get_mut<T: Any>(&mut self) -> &mut T {
        self.store
            .get_mut(&TypeId::of::<T>())
            .unwrap_or_else(|| panic!("Data expected from AppContextGenHandler of a type that hasn't been entered.\nT = {}", std::any::type_name::<T>()))
            .downcast_mut()
            .unwrap()
    }

    pub fn overwrite<T: Any>(&mut self, value: T) {
        *self.get_mut::<T>() = value;
    }

    #[allow(private_bounds)]
    pub fn set<T: Any + DerefMutSized>(&mut self, value: T::Target) {
        **self.get_mut::<T>() = value;
    }

    #[allow(private_bounds)]
    pub fn value<T: Any + DerefCopy>(&self) -> T::Target {
        **self.get::<T>()
    }
}

trait DerefMutSized: DerefMut<Target = Self::T> {
    type T: Sized;
}

impl<T: DerefMut<Target = I>, I: Sized> DerefMutSized for T {
    type T = T::Target;
}

trait DerefCopy: DerefMut<Target = Self::T> {
    type T: Sized + Copy;
}

impl<T: DerefMut<Target = I>, I: Sized + Copy> DerefCopy for T {
    type T = T::Target;
}

pub struct AppContextGenHandler {
    store: Store
}

impl AppContextGenHandler {
    fn new() -> Self {
        Self {
            store: Store::new()
        }
    }

    pub fn push<T: Any>(&mut self, data: T) {
        self.push_boxed(Box::new(data));
    }
    
    pub fn push_boxed<T: Any>(&mut self, data: Box<T>) {
        if self.store.store.insert(TypeId::of::<T>(), data).is_some() {
            panic!("Data of the same type given to AppContextGenHandler.")
        }
    }

    fn into_context(self, user_inputs: UserInputs) -> AppContext {
        AppContext { user_inputs, store: self.store }
    }
}

impl Deref for AppContextGenHandler {
    type Target = Store;
    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

impl DerefMut for AppContextGenHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.store
    }
}


pub fn weak_from_opt<T: Node + 'static>(this: &Option<WeakRef<T>>) -> WeakRef<dyn Node> {
    let result = this.clone().unwrap();
    result as WeakRef<dyn Node>
}

pub fn weak_from<T: Node + 'static>(this: &WeakRef<T>) -> WeakRef<dyn Node> {
    let result = this.clone();
    result as WeakRef<dyn Node>
}

pub fn contains_node(vec: &Vec<WeakRef<dyn Node>>, test: &WeakRef<dyn Node>) -> bool {
    let Some(upgraded) = &test.upgrade() else { return false; };
    vec.iter().any(|d|
        if let Some(a) = d.upgrade() {
            Rc::ptr_eq(&a, upgraded)
        } else { false }
    )
}

pub type StrongRef<T> = Rc<RefCell<T>>;
pub type WeakRef<T> = Weak<RefCell<T>>;
pub type WeakNode = Weak<NodeStore>;
pub type StrongNode = Rc<NodeStore>;

/// # Example usage:
/// ```no_run
/// use super::*;
///
/// pub struct Example {
///
/// }
///
/// impl NewNoOut for Example {
///     type InType = ();
///     fn new(_: Self::InType, _handler: &mut GenHandler) -> Self {
///         Self {
///
///         }
///     }
/// }
///
/// impl Node for Example {
///     fn update(&mut self, user_inputs: &UserInputs, node: &NodeStore) {
///
///     }
///
///     fn hit_detect(&mut self, pos: Vec2, node: &NodeStore) -> Vec<WeakNode> {
///         if self.rect.contains(pos) {
///             vec![node.get_weak()]
///         } else {
///             vec![]
///         }
///     }
/// }
/// ```
#[allow(private_bounds)]
pub trait Node: AsAny {
    fn update(&mut self, user_inputs: &mut AppContextHandler, node: &NodeStore);
    fn hit_detect(&mut self, pos: Vec2, node: &NodeStore, store: &mut Store) -> Vec<WeakNode>;
}

trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[allow(private_bounds)]
pub trait AsAnyExt: AsAny {
    fn try_get_self_mut<T: Node + 'static>(&mut self) -> Option<&mut T>;
    fn try_get_self<T: Node + 'static>(&self) -> Option<&T>;
    fn get_self<T: Node + 'static>(&self) -> &T;
    fn get_self_mut<T: Node + 'static>(&mut self) -> &mut T;
}

impl<A: Any + Node> AsAny for A {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<A: AsAny> AsAnyExt for A {
    fn try_get_self<T: Node + 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }

    fn try_get_self_mut<T: Node + 'static>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut()
    }

    fn get_self<T: Node + 'static>(&self) -> &T {
        self.try_get_self().unwrap_or_else(|| panic!("In `get_self`, {DOWNCAST_FAILED}"))
    }

    fn get_self_mut<T: Node + 'static>(&mut self) -> &mut T {
        self.try_get_self_mut().unwrap_or_else(|| panic!("In `get_self_mut`, {DOWNCAST_FAILED}"))
    }
}

pub trait NewInOut {
    type InType;
    type OutType;
    fn new(data: Self::InType, handler: &mut GenHandler) -> (Self::OutType, Self);
}

pub trait NewNoOut {
    type InType;
    fn new(data: Self::InType, handler: &mut GenHandler) -> Self;
}

impl<T: NewNoOut> NewInOut for T {
    type InType = T::InType;
    type OutType = ();
    fn new(data: Self::InType, handler: &mut GenHandler) -> (Self::OutType, Self) {
        ((), T::new(data, handler))
    }
}

pub trait New {
    fn new(handler: &mut GenHandler) -> Self;
}

impl<T: New> NewNoOut for T {
    type InType = ();
    fn new(_: Self::InType, handler: &mut GenHandler) -> Self {
        T::new(handler)
    }
}

pub struct GenHandler<'a> {
    children: Vec<Rc<NodeStore>>,
    ctx: &'a mut AppContextGenHandler
}

pub struct Handler<'a> {
    this: &'a NodeStore,
}

impl<'a> Handler<'a> {
    fn new(this: &'a NodeStore) -> Self {
        Handler {
            this,
        }
    }
}

impl<'a> Handler<'a> {
    pub fn hit_detect(&self, pos: Vec2, store: &mut Store) -> Vec<WeakNode> {
        self.this.node.borrow_mut().hit_detect(pos, self.this, store)
    }

    pub fn update(&self, ctx: &mut AppContextHandler) {
        self.this.node.borrow_mut().update(ctx, self.this);
    }

    pub fn get_self_dyn<T, F: FnOnce(&dyn Node) -> T>(&self, f: F) -> T {
        f(self.this.node.borrow().as_ref())
    }

    pub fn get_self_dyn_mut<T, F: FnOnce(&mut dyn Node) -> T>(&self, f: F) -> T {
        f(self.this.node.borrow_mut().as_mut())
    }
}

impl<'a> Deref for Handler<'a> {
    type Target = NodeStore;
    fn deref(&self) -> &Self::Target {
        self.this
    }
}

pub struct NodeStore {
    this: WeakNode,
    node: RefCell<Box<dyn Node>>,
    children: Vec<StrongNode>
}

pub struct Origin {
    node: StrongNode
}

impl Origin {
    pub fn get_handler(&'_ self) -> Handler<'_> {
        Handler::new(&self.node)
    }
}

const SELF_THIS_INVALID: &str = "pointer to self dropped and upgrade impossible. This means `self.this` did not point to self.";
const INVALID_WEAK: &str = "pointer dropped and upgrade impossible.";
const DOWNCAST_FAILED: &str = "downcast failed. Either tree is malformed, or an incorrect type was supplied.";

impl GenHandler<'_> {
    pub fn push_child<Child: Node + New + 'static>(&mut self) {
        self.push_child_io::<Child>(())
    }

    pub fn push_child_io<Child: Node + NewInOut + 'static>(&mut self, data: Child::InType) -> Child::OutType {
        let (out, node) = NodeStore::new::<Child>(data, self.ctx);
        self.children.push(
            node
        );
        out
    }

    pub fn push_data<T: Any>(&mut self, data: T) {
        self.ctx.push(data);
    }

    pub fn push_data_boxed<T: Any>(&mut self, data: Box<T>) {
        self.ctx.push_boxed(data);
    }
}

impl NodeStore {
    pub fn origin<T: Node + NewInOut<InType = (), OutType = ()> + 'static>(ctx: &mut AppContextGenHandler) -> Origin {
        Origin {node: Self::new::<T>((), ctx).1 }
    }

    fn new<Child: Node + NewInOut + 'static>(data: Child::InType, ctx: &mut AppContextGenHandler) -> (Child::OutType, StrongNode) {
        let mut handler = GenHandler { children: vec![], ctx };
        let (out, node) = Child::new(data, &mut handler);
        let node = RefCell::new(Box::new(node));
        (
            out,
            Rc::new_cyclic(|me| {
                Self {
                    this: me.clone(),
                    children: handler.children,
                    node
                }
            })
        )
    }

    pub fn get_children(&'_ self) -> Vec<Handler<'_>>{
        self.children.iter().map(|c| Handler::new(c.as_ref())).collect()
    }

    pub fn update_children(&self, user_inputs: &mut AppContextHandler) {
        for child in self.get_children() {
            child.update(user_inputs)
        }
    }

    pub fn hit_detect_children_and_self(&self, pos: Vec2, store: &mut Store) -> Vec<WeakNode>{
        let mut result = vec![];
        for child in self.get_children() {
            result = child.hit_detect(pos, store);
            if !result.is_empty() {
                break
            }
        }
        result.push(self.get_weak());
        result
    }

    fn contains_self(&self, vec: &[WeakNode]) -> bool {
        let this = self.this.clone().upgrade().unwrap_or_else(|| panic!("In `contains_self`, {SELF_THIS_INVALID}"));
        vec.iter().any(|d|
            Rc::ptr_eq(&this, &d.upgrade().unwrap_or_else(|| panic!("In `contains_self`, {INVALID_WEAK}")))
        )
    }

    pub fn get_weak(&self) -> WeakNode {
        self.this.clone()
    }
}

pub struct Frame {
    origin: Origin,
    ctx: AppContext
}

impl Frame {
    pub fn new<T: Node + NewInOut<InType = (), OutType = ()> + 'static>() -> Self {
        let mut ctx = AppContextGenHandler::new();
        let origin = NodeStore::origin::<T>(&mut ctx);
        let user_inputs = UserInputs::new(&origin);
        let ctx = ctx.into_context(user_inputs);
        Self {
            origin,
            ctx
        }
    }

    pub async fn update(&mut self) {
        self.ctx.user_inputs.update(&mut self.ctx.store);
        self.origin.get_handler().update(&mut self.ctx.get_handler());
        next_frame().await;
    }
}