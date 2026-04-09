pub struct Inventory {
    pub items: [ItemSlot; 16],
}

#[derive(Debug, Clone)]
pub struct ItemSlot {
    pub item: Option<ItemDef>,
    pub quantity: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemDef {
    pub name: String,
    pub data: Vec<u8>,
}
