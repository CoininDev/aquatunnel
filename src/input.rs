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
    /// Creates a new `InputContext` with zeroed movement and look directions, an empty action queue, and the specified input setup.
    ///
    /// # Examples
    ///
    /// ```
    /// let setup = InputSetup::default();
    /// let ctx = InputContext::new(setup);
    /// assert_eq!(ctx.move_direction, Vec2::ZERO);
    /// assert_eq!(ctx.look_direction, Vec2::ZERO);
    /// assert!(ctx.actions.is_empty());
    /// ```
    pub fn new(setup: InputSetup) -> Self {
        InputContext {
            move_direction: Vec2::ZERO,
            look_direction: Vec2::ZERO,
            actions: VecDeque::new(),
            setup,
        }
    }

    /// Updates the movement and look direction vectors based on the configured input methods.
    ///
    /// Selects and applies the input methods specified by `move_method` and `look_method` in the input setup,
    /// updating `move_direction` and `look_direction` accordingly. If an invalid method index is provided,
    /// the corresponding direction is set to a zero vector.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut ctx = InputContext::new(InputSetup::default());
    /// ctx.update();
    /// // ctx.move_direction and ctx.look_direction are now set based on input
    /// ```
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

    /// Computes a normalized 2D movement vector based on the current state of the WASD keys.
    ///
    /// Returns a vector representing the direction indicated by the pressed keys:
    /// - W/S control the vertical axis (negative/positive Y).
    /// - A/D control the horizontal axis (negative/positive X).
    /// If no keys are pressed, returns a zero vector.
    ///
    /// # Examples
    ///
    /// ```
    /// let ctx = InputContext::new(InputSetup::default());
    /// let dir = ctx.method0();
    /// // If no keys are pressed, dir == Vec2::ZERO
    /// ```
    fn method0(&self) -> Vec2 {
        // -A +D
        let x_signal = -(is_key_down(KeyCode::A) as i32) + (is_key_down(KeyCode::D) as i32);
        // -W +S
        let y_signal = -(is_key_down(KeyCode::W) as i32) + (is_key_down(KeyCode::S) as i32);

        Vec2::new(x_signal as f32, y_signal as f32).normalize_or_zero()
    }

    /// Returns a normalized vector pointing from the center of the screen to the current mouse position.
    ///
    /// The resulting vector represents the direction from the screen center toward the mouse cursor. If the mouse is exactly at the center, a zero vector is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// let ctx = InputContext::new(InputSetup::default());
    /// let dir = ctx.method1();
    /// // dir is a normalized Vec2 pointing from screen center to mouse position
    /// ```
    fn method1(&self) -> Vec2 {
        let (mouse_x, mouse_y) = mouse_position();
        let center_x = screen_width() / 2.0;
        let center_y = screen_width() / 2.0;

        let direction = Vec2::new(mouse_x - center_x, mouse_y - center_y);
        direction.normalize_or_zero()
    }

    /// Computes a normalized movement vector based on the state of the arrow keys.
    ///
    /// Returns a 2D vector representing the direction indicated by the pressed arrow keys:
    /// Left/Right control the x-axis, Up/Down control the y-axis. The resulting vector is normalized,
    /// or zero if no arrow keys are pressed.
    ///
    /// # Examples
    ///
    /// ```
    /// let ctx = InputContext::new(InputSetup::default());
    /// let dir = ctx.method2();
    /// // If no arrow keys are pressed, dir == Vec2::ZERO
    /// ```
    fn method2(&self) -> Vec2 {
        // -left +right
        let x_signal = -(is_key_down(KeyCode::Left) as i32) + (is_key_down(KeyCode::Right) as i32);
        // -up +down
        let y_signal = -(is_key_down(KeyCode::Up) as i32) + (is_key_down(KeyCode::Down) as i32);

        Vec2::new(x_signal as f32, y_signal as f32).normalize_or_zero()
    }

    /// Returns the normalized delta movement vector of the mouse since the last frame.
    ///
    /// The resulting vector represents the direction and magnitude of mouse movement, normalized to unit length. If there is no mouse movement, returns a zero vector.
    ///
    /// # Examples
    ///
    /// ```
    /// let ctx = InputContext::new(InputSetup::default());
    /// let delta = ctx.method3();
    /// // delta is a normalized Vec2 representing mouse movement direction
    /// ```
    fn method3(&self) -> Vec2 {
        mouse_delta_position().normalize_or_zero()
    }
    /// ```
    fn method4(&self) -> Vec2 {
        todo!()
    }
    /// ```
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
    /// Creates a default input setup with standard keybindings and input methods.
    ///
    /// The default configuration maps the D key and left mouse button to the debug action,
    /// uses WASD keys for movement, and sets looking direction based on the mouse position relative to the screen center.
    ///
    /// # Examples
    ///
    /// ```
    /// let setup = InputSetup::default();
    /// assert_eq!(setup.move_method, 0);
    /// assert_eq!(setup.look_method, 1);
    /// assert!(setup.keybindings.contains_key(&RawAction::Key(KeyCode::D)));
    /// ```
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
