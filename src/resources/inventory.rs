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

impl Inventory {
    pub fn add_item(&mut self, item: ItemDef, quantity: u8) -> Result<(), ()> {
        // Find existing stack
        if let Some(slot) = self.items.iter_mut().find(|s| s.item.as_ref() == Some(&item)) {
            slot.quantity += quantity;
            return Ok(());
        }
        
        // Find empty slot or push
        if self.items.len() < self.max as usize {
            self.items.push(ItemSlot { item: Some(item), quantity });
            return Ok(());
        }

        Err(())
    }

    pub fn remove_item(&mut self, item_def: ItemDef, quantity: u8) -> Result<(), ()> {
        // Find existing stack
        if let Some((idx, slot)) = self.items.iter_mut().enumerate().find(|(_, s)| s.item.as_ref() == Some(&item_def)) {
            if slot.quantity < quantity {
                return Err(());
            }
            slot.quantity -= quantity;
            if slot.quantity == 0 {
                self.items.remove(idx);
            }
            return Ok(());
        }

        Err(())
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
