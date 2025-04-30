use std::collections::{HashMap, VecDeque};

use macroquad::input::{KeyCode, MouseButton, is_key_down};
use macroquad::input::{mouse_delta_position, mouse_position};
use macroquad::math::Vec2;
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
        self.move_direction = match self.setup.move_method {
            0 => self.method0(),
            1 => self.method1(),
            2 => self.method2(),
            3 => self.method3(),
            4 => self.method4(),
            5 => self.method5(),
            _ => Vec2::ZERO,
        };

        self.look_direction = match self.setup.look_method {
            0 => self.method0(),
            1 => self.method1(),
            2 => self.method2(),
            3 => self.method3(),
            4 => self.method4(),
            5 => self.method5(),
            _ => Vec2::ZERO,
        };
    }

    // WASD
    fn method0(&self) -> Vec2 {
        // -A +D
        let x_signal = -(is_key_down(KeyCode::A) as i32) + (is_key_down(KeyCode::D) as i32);
        // -W +S
        let y_signal = -(is_key_down(KeyCode::W) as i32) + (is_key_down(KeyCode::S) as i32);

        Vec2::new(x_signal as f32, y_signal as f32).normalize_or_zero()
    }

    // Direção do centro da tela pro mouse
    fn method1(&self) -> Vec2 {
        let (mouse_x, mouse_y) = mouse_position();
        let center_x = screen_width() / 2.0;
        let center_y = screen_width() / 2.0;

        let direction = Vec2::new(mouse_x - center_x, mouse_y - center_y);
        direction.normalize_or_zero()
    }

    // Setas do teclado
    fn method2(&self) -> Vec2 {
        // -left +right
        let x_signal = -(is_key_down(KeyCode::Left) as i32) + (is_key_down(KeyCode::Right) as i32);
        // -up +down
        let y_signal = -(is_key_down(KeyCode::Up) as i32) + (is_key_down(KeyCode::Down) as i32);

        Vec2::new(x_signal as f32, y_signal as f32).normalize_or_zero()
    }

    // Direção delta do mouse
    fn method3(&self) -> Vec2 {
        mouse_delta_position().normalize_or_zero()
    }
    // Joystick esquerdo
    fn method4(&self) -> Vec2 {
        todo!()
    }
    // Joystick direito
    fn method5(&self) -> Vec2 {
        todo!()
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
        keybindings.insert(
            RawAction::MouseButton(MouseButton::Left),
            InputAction::DebugAction,
        );

        InputSetup {
            keybindings,
            move_method: 0,
            look_method: 1,
        }
    }
}
