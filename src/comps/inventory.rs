use crate::resources::inventory::{Inventory, ItemDef};

#[derive(Debug, Clone, PartialEq)]
pub struct InventoryHolder {
    pub inventory: Inventory,
}

use macroquad::math::IVec2;

#[derive(Debug, Clone, PartialEq)]
pub struct WorldItem {
    pub item: ItemDef,
    pub quantity: u8,
    pub chunk: IVec2,
    pub active: bool,
}
