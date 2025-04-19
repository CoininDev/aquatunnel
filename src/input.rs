use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

use glam::Vec2;
use sdl2::{EventPump, event::Event, keyboard::Keycode};

pub struct InputContext {
    pub move_direction: Vec2,
    pub look_direction: Vec2,
    pub actions: VecDeque<InputAction>,
    setup: InputSetup,
    event_pump: Rc<RefCell<EventPump>>,
}

impl InputContext {
    pub fn new(event_pump: Rc<RefCell<EventPump>>, setup: InputSetup) -> Self {
        InputContext {
            move_direction: Vec2::ZERO,
            look_direction: Vec2::ZERO,
            actions: VecDeque::new(),
            setup,
            event_pump,
        }
    }

    pub fn update(&mut self) {
        // Essa função deve ler inputs brutos e atualizar as informações move_direction,
        // look_direction e actions, para serem exportadas à lógica do jogo.

        for event in self.event_pump.borrow_mut().poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(k), ..
                } => {
                    if let Some(action) = self.setup.keybindings.get(&RawAction::Key(k)) {
                        self.actions.push_back(action.clone());
                    }
                }

                _ => {}
            }
        }

        match self.setup.move_method {
            0 => self.move_direction = self.method_0(),
            _ => {}
        }
    }

    fn method_0(&self) -> Vec2 {
        let event_pump_ref = self.event_pump.borrow_mut();
        let keyboard_state = event_pump_ref.keyboard_state();
        // -A +D
        let x_signal = -(keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::A) as i32)
            + (keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::D) as i32);
        // -W +S
        let y_signal = -(keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::W) as i32)
            + (keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::S) as i32);

        Vec2::new(x_signal as f32, y_signal as f32).normalize_or_zero()
    }
}

#[derive(Clone, Copy)]
pub enum InputAction {
    DebugAction,
}

#[derive(PartialEq, Eq, Hash)]
pub enum RawAction {
    Key(Keycode),
}

pub struct InputSetup {
    keybindings: HashMap<RawAction, InputAction>,
    move_method: u8,
    look_method: u8,
}

impl Default for InputSetup {
    fn default() -> Self {
        let mut keybindings = HashMap::new();
        keybindings.insert(RawAction::Key(Keycode::D), InputAction::DebugAction);

        InputSetup {
            keybindings,
            move_method: 0,
            look_method: 1,
        }
    }
}
