use legion::{Entity, systems::CommandBuffer, world::SubWorld};
use crate::comps::InventoryHolder;

#[derive(Debug, Clone, PartialEq)]
pub struct Inventory {
    pub max: u64,
    pub items: Vec<ItemSlot>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self { 
            max: 8, 
            items: Default::default() 
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemSlot {
    pub item: Option<ItemDef>,
    pub quantity: u8,
}

#[derive(Clone)]
pub struct ItemContext<'a> {
    pub holder: &'a InventoryHolder,
    pub holder_entity: Entity,
    pub world: &'a SubWorld<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemDef {
    pub name: String,
    pub data: Vec<u8>,
    pub use_func : Option<fn(&mut CommandBuffer,&ItemContext)>
}
