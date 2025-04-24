use std::collections::{HashMap, VecDeque};

use macroquad::math::Vec2;
use macroquad::input::{is_key_down, KeyCode, MouseButton};
use macroquad::input::mouse_position;
use macroquad::window::screen_width;
pub struct InputContext {
    pub move_direction: Vec2,
    pub look_direction: Vec2,
    pub actions: VecDeque<InputAction>,
    pub setup: InputSetup,
}

impl InputContext {
    pub fn new(setup: InputSetup) -> Self {
        InputContext {
            move_direction: Vec2::ZERO,
            look_direction: Vec2::ZERO,
            actions: VecDeque::new(),
            setup,
        }
    }

    pub fn update(&mut self) {
        match self.setup.move_method {
            0 => self.move_direction = self.method_0(),
            1 => self.move_direction = self.method_1(),
            _ => {}
        }

        match self.setup.look_method {
            0 => self.look_direction = self.method_0(),
            1 => self.look_direction = self.method_1(),
            _ => {}
        }
    }

    fn method_0(&self) -> Vec2 {
        // -A +D
        let x_signal = -(is_key_down(KeyCode::A) as i32)
            + (is_key_down(KeyCode::D) as i32);
        // -W +S
        let y_signal = -(is_key_down(KeyCode::W) as i32)
            + (is_key_down(KeyCode::S) as i32);

        Vec2::new(x_signal as f32, y_signal as f32).normalize_or_zero()
    }

    fn method_1(&self) -> Vec2 {
        let (mouse_x, mouse_y) = mouse_position();
        let center_x = screen_width() / 2.0;
        let center_y = screen_width() / 2.0;

        let direction = Vec2::new(mouse_x - center_x, mouse_y - center_y);
        direction.normalize_or_zero()
    }
}

#[derive(Clone, Copy)]
pub enum InputAction {
    DebugAction,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum RawAction {
    Key(KeyCode),
    MouseButton(MouseButton),
}

#[derive(Clone)]
pub struct InputSetup {
    pub keybindings: HashMap<RawAction, InputAction>,
    move_method: u8,
    look_method: u8,
}

impl Default for InputSetup {
    fn default() -> Self {
        let mut keybindings = HashMap::new();
        keybindings.insert(RawAction::Key(KeyCode::D), InputAction::DebugAction);
        keybindings.insert(RawAction::MouseButton(MouseButton::Left), InputAction::DebugAction);

        InputSetup {
            keybindings,
            move_method: 0,
            look_method: 1,
        }
    }
}
