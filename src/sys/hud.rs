use egui_macroquad::egui::{self, Ui, ahash::{HashMap, HashMapExt}};
use macroquad::math::Vec2;
use macroquad::math::IVec2;
use legion::{world::SubWorld, systems::CommandBuffer, *};
use crate::resources::inventory::ItemContext;
use crate::{
    comps::*,
    resources::{
        gui_commands::{GuiCommand, GuiCommandBuffer},
        input::InputContext,
    },
};

#[system]
#[write_component(Window)]
pub fn load_windows(
    world: &mut SubWorld,
    #[resource] input: &mut InputContext,
) {
    let mut builds: HashMap<String, Box<dyn Fn(&mut Ui) + Send + Sync + 'static >> = HashMap::new();
    let mut windows: Vec<&mut Window> = Vec::new();

    <&mut Window>::query()
        .iter_mut(world)
        .for_each(|x| {
            if let Some(f) = x.build_func.take() {
                builds.insert(x.title.clone(), f);
                windows.push(x);
            }
        });

    egui_macroquad::ui(|ctx| {
        input.lock_mouse = ctx.wants_pointer_input();
        input.lock_keybd = ctx.wants_keyboard_input();

        for (t, b) in builds.iter() {
            egui::Window::new(t).show(ctx, b);
        }
    });

    for w in windows {
        if let Some(f) = builds.remove(&w.title) {
            w.build_func = Some(f);
        }
    }
}

#[system]
pub fn render_egui(
) {
    egui_macroquad::draw();
}

#[system]
#[write_component(InventoryHolder)]
#[read_component(InventoryHolder)]
pub fn process_gui_commands(
    world: &mut SubWorld,
    #[resource] gui_cmds: &mut GuiCommandBuffer,
    cb: &mut CommandBuffer,
) {
    let commands = gui_cmds.drain();
    if commands.is_empty() { return; }

    let mut drops: Vec<(crate::resources::inventory::ItemDef, u8, Vec2)> = Vec::new();
    let mut uses: Vec<crate::resources::inventory::ItemDef> = Vec::new();

    for cmd in commands {
        match cmd {
            GuiCommand::DropItem { item, quantity, spawn_pos } => drops.push((item, quantity, spawn_pos)),
            GuiCommand::UseItem { item } => uses.push(item),
        }
    }
    
    if !drops.is_empty() {
        let mut inv_q = <&mut InventoryHolder>::query();
        if let Some(holder) = inv_q.iter_mut(world).next() {
            for (item, quantity, spawn_pos) in drops {
                let _ = holder.inventory.remove_item(item.clone(), quantity);

                let chunk = IVec2::new(
                    (spawn_pos.x / 16.0).floor() as i32,
                    (spawn_pos.y / 16.0).floor() as i32,
                );
                cb.push((
                    crate::comps::Transform {
                        position: spawn_pos,
                        ..Default::default()
                    },
                    crate::comps::Body::new(Vec2::new(0.1, 0.1), true),
                    crate::comps::Sprite {
                        image_path: "assets/gun.png".into(),
                        z_order: 1.0,
                        flip_x: false,
                        flip_y: false,
                    },
                    crate::comps::WorldItem {
                        item,
                        quantity,
                        chunk,
                        active: true,
                    },
                ));
            }
        }
    }

    if !uses.is_empty() {
        let holder_entity = <(Entity, &InventoryHolder)>::query()
            .iter(world)
            .next()
            .map(|(e, _)| *e);

        if let Some(holder_entity) = holder_entity {
            let mut inv_q = <&InventoryHolder>::query();
            if let Ok(holder) = inv_q.get(world, holder_entity) {
                let item_ctx = ItemContext {
                    holder,
                    holder_entity,
                    world,
                };
                for item in uses {
                    if let Some(f) = item.use_func {
                        f(cb, &item_ctx);
                    }
                }
            }
        }
    }
}
