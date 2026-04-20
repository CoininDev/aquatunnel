use macroquad::math::Vec2;
use crate::resources::inventory::ItemDef;

/// Actions that the UI layer can request.
/// These are queued each frame and executed by `process_gui_commands_system`.
#[derive(Clone, Debug)]
pub enum GuiCommand {
    DropItem { item: ItemDef, quantity: u8, spawn_pos: Vec2 },
    UseItem { item: ItemDef },
}

/// A resource that acts as a queue of UI-originated commands.
/// The UI pushes commands here; a dedicated ECS system drains and executes them.
#[derive(Default)]
pub struct GuiCommandBuffer {
    pub commands: Vec<GuiCommand>,
}

impl GuiCommandBuffer {
    pub fn push(&mut self, cmd: GuiCommand) {
        self.commands.push(cmd);
    }

    /// Drain all pending commands for processing.
    pub fn drain(&mut self) -> Vec<GuiCommand> {
        std::mem::take(&mut self.commands)
    }
}
