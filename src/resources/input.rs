use std::collections::{HashMap, HashSet};

use macroquad::input::{
    KeyCode, MouseButton, is_key_down, is_key_pressed, is_key_released, is_mouse_button_down, is_mouse_button_released
};
use macroquad::input::{mouse_delta_position, mouse_position};
use macroquad::math::Vec2;
use macroquad::window::{screen_height, screen_width};

pub struct InputContext {
    pub move_direction: Vec2,
    pub look_direction: Vec2,
    pub actions: HashSet<InputAction>,
    pub setup: InputSetup,
    
    // egui window locks input
    pub lock_mouse: bool,
    pub lock_keybd: bool,
}

impl InputContext {
    pub fn new(setup: InputSetup) -> Self {
        InputContext {
            move_direction: Vec2::ZERO,
            look_direction: Vec2::ZERO,
            actions: HashSet::new(),
            lock_mouse: false,
            lock_keybd: false,
            setup,
        }
    }

    pub fn update(&mut self) {
        let m = MethodContext::new(self.lock_mouse, self.lock_keybd);
        self.move_direction = self.setup.move_method.run(&m);
        self.look_direction = self.setup.look_method.run(&m);


        self.actions.clear();
        for (raw, action) in &self.setup.keybindings {
            let is_pressed = match raw {
                RawAction::Key(k)           if !self.lock_keybd => is_key_down(*k),
                RawAction::KeyOnce(k)       if !self.lock_keybd => is_key_pressed(*k),
                RawAction::MouseButton(b)   if !self.lock_mouse => is_mouse_button_down(*b),
                RawAction::KeyUp(k)         if !self.lock_keybd => is_key_released(*k),
                RawAction::MouseButtonUp(b) if !self.lock_mouse => is_mouse_button_released(*b),
                _ => false
            };

            if is_pressed {
                self.actions.insert(*action);
            }
        }
    }

    pub fn consume_action(&mut self, target: InputAction) -> bool {
        self.actions.take(&target).is_some()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputAction {
    DebugActionOn,
    DebugActionOff,
    InventoryToggle,
    Interact,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum RawAction {
    Key(KeyCode),
    KeyUp(KeyCode),
    KeyOnce(KeyCode),
    MouseButton(MouseButton),
    MouseButtonUp(MouseButton),
}
pub struct InputSetup {
    pub keybindings: HashMap<RawAction, InputAction>,
    move_method: Box<dyn AxisMethod>,
    look_method: Box<dyn AxisMethod>,
}

impl Default for InputSetup {
    fn default() -> Self {
        let mut keybindings = HashMap::new();
        keybindings.insert(RawAction::KeyOnce(KeyCode::F), InputAction::Interact);
        keybindings.insert(
            RawAction::MouseButton(MouseButton::Left),
            InputAction::DebugActionOn,
        );
        keybindings.insert(
            RawAction::MouseButtonUp(MouseButton::Left),
            InputAction::DebugActionOff,
        );

        keybindings.insert(
            RawAction::KeyOnce(KeyCode::E), 
            InputAction::InventoryToggle
        );

        InputSetup {
            keybindings,
            move_method: Box::new(WASDMethod),
            look_method: Box::new(MouseCenterMethod),
        }
    }
}

/// Uma forma de conseguir uma direção
/// Pode ser baseada no mouse, ou no WASD, ou num joystick.
struct MethodContext {
    lock_mouse: bool,
    lock_keybd: bool
}

impl MethodContext {
    fn new(lock_mouse: bool, lock_keybd: bool) -> MethodContext {
        MethodContext { lock_mouse, lock_keybd }
    }
}

trait AxisMethod {
    fn run(&self, m: &MethodContext) -> Vec2;
}

#[derive(Clone)]
struct WASDMethod;
impl AxisMethod for WASDMethod {
    fn run(&self, m: &MethodContext) -> Vec2 {
        if m.lock_keybd { return Vec2::ZERO; }

        // -A+D
        let x_signal = -(is_key_down(KeyCode::A) as i32) + (is_key_down(KeyCode::D) as i32);
        // -W+S
        let y_signal = -(is_key_down(KeyCode::W) as i32) + (is_key_down(KeyCode::S) as i32);

        Vec2::new(x_signal as f32, y_signal as f32).normalize_or_zero()
    }
}

#[derive(Clone)]
struct MouseCenterMethod;
impl AxisMethod for MouseCenterMethod {
    fn run(&self, m: &MethodContext) -> Vec2 {
        if m.lock_mouse { return Vec2::ZERO; }

        let (mouse_x, mouse_y) = mouse_position();
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        let direction = Vec2::new(mouse_x - center_x, mouse_y - center_y);
        direction.normalize_or_zero()
    }
}

#[derive(Clone)]
struct MouseDeltaMethod;
impl AxisMethod for MouseDeltaMethod {
    fn run(&self, m: &MethodContext) -> Vec2 {
        if m.lock_mouse { return Vec2::ZERO; }
        -mouse_delta_position().normalize_or_zero()
    }
}

#[derive(Clone)]
struct ArrowsMethod;
impl AxisMethod for ArrowsMethod {
    fn run(&self, m: &MethodContext) -> Vec2 {
        if m.lock_keybd { return Vec2::ZERO; }

        // -left +right
        let x_signal = -(is_key_down(KeyCode::Left) as i32) + (is_key_down(KeyCode::Right) as i32);
        // -up +down
        let y_signal = -(is_key_down(KeyCode::Up) as i32) + (is_key_down(KeyCode::Down) as i32);

        Vec2::new(x_signal as f32, y_signal as f32).normalize_or_zero()
    }
}
