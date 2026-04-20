use egui_macroquad::egui;
use legion::{world::SubWorld, systems::CommandBuffer, *};
use crate::{
    comps::*,
    resources::{
        gui_commands::{GuiCommand, GuiCommandBuffer},
        input::{InputAction, InputContext},
        physics::PhysicsContext,
    },
};

#[system]
#[read_component(Transform)]
#[read_component(Player)]
#[write_component(InventoryHolder)]
#[read_component(WorldItem)]
#[read_component(Body)]
pub fn interact_pickup(
    world: &mut SubWorld,
    cmd: &mut CommandBuffer,
    #[resource] input: &mut InputContext,
    #[resource] physics: &mut PhysicsContext,
) {
    if !input.consume_action(InputAction::Interact) {
        return;
    }

    let mut player_pos = None;
    let mut player_entity = None;

    let mut player_q = <(Entity, &Transform, &Player)>::query();
    for (entity, transform, _) in player_q.iter(world) {
        player_pos = Some(transform.position);
        player_entity = Some(*entity);
        break;
    }

    let player_pos = if let Some(p) = player_pos { p } else { return };
    let player_entity = if let Some(e) = player_entity { e } else { return };

    let mut items_to_pickup = Vec::new();
    let mut item_q = <(Entity, &Transform, &WorldItem, Option<&Body>)>::query();
    for (entity, item_transform, world_item, body) in item_q.iter(world) {
        if world_item.active && item_transform.position.distance(player_pos) < 1.5 {
            items_to_pickup.push((*entity, world_item.item.clone(), world_item.quantity, body.cloned()));
        }
    }

    if items_to_pickup.is_empty() { return; }

    let mut inv_q = <&mut InventoryHolder>::query();
    if let Ok(holder) = inv_q.get_mut(world, player_entity) {
        for (item_entity, item_def, quantity, body_opt) in items_to_pickup {
            if holder.inventory.add_item(item_def, quantity).is_ok() {
                cmd.remove(item_entity);
                if let Some(body) = body_opt {
                    if let Some(handle) = body.body_handle {
                        physics.bodies.remove(
                            handle,
                            &mut physics.islands,
                            &mut physics.colliders,
                            &mut physics.impulse_joints,
                            &mut physics.multibody_joints,
                            true,
                        );
                    }
                }
            }
        }
    }
}

#[system]
#[read_component(InventoryHolder)]
#[read_component(Player)]
#[read_component(Transform)]
#[write_component(Window)]
pub fn inventory_window(
    #[state] open: &mut bool,
    // Persists across frames so the closure (executed next frame) can fill it
    #[state] pending: &mut std::sync::Arc<std::sync::Mutex<Vec<GuiCommand>>>,
    #[resource] input: &mut InputContext,
    #[resource] gui_cmds: &mut GuiCommandBuffer,
    world: &mut SubWorld,
) {
    *open ^= input.consume_action(InputAction::InventoryToggle);

    // Capture player position to use as drop spawn point (captured into closure)
    let player_pos = <(&Transform, &Player)>::query()
        .iter(world)
        .next()
        .map(|(t, _)| t.position)
        .unwrap_or_default();

    // Drain commands queued by the closure in the previous frame
    if let Ok(mut q) = pending.try_lock() {
        for cmd in q.drain(..) {
            gui_cmds.push(cmd);
        }
    }

    let holder_data = <(&InventoryHolder, &mut Window)>::query()
        .iter_mut(world)
        .next();

    if let Some((holder, window)) = holder_data {
        if *open {
            let items = holder.inventory.items.clone();
            let items_count = holder.inventory.items.iter().filter(|a| a.item.is_some()).count();
            let items_max = holder.inventory.max;

            // Clone the persistent Arc so the 'static closure can push into it
            let pending_clone = std::sync::Arc::clone(pending);

            window.build_func = Some(Box::new(move |ui| {
                ui.label(egui::RichText::new("Inventário").strong().size(16.0));
                ui.separator();

                egui::Grid::new("inventory_grid")
                    .num_columns(4)
                    .spacing([32.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Usar").underline());
                        ui.label(egui::RichText::new("Item").underline());
                        ui.label(egui::RichText::new("Qtd.").underline());
                        ui.label(egui::RichText::new("Dropar").underline());
                        ui.end_row();

                        for item in items.clone() {
                            if let Some(item_def) = item.item {
                                let name = item_def.name.clone();
                                if ui.button("▶ Usar").clicked() {
                                    if let Ok(mut q) = pending_clone.try_lock() {
                                        q.push(GuiCommand::UseItem { item: item_def.clone() });
                                    }
                                }
                                ui.label(&name);
                                ui.label(item.quantity.to_string());
                                if ui.button("✖ Drop").clicked() {
                                    if let Ok(mut q) = pending_clone.try_lock() {
                                        q.push(GuiCommand::DropItem {
                                            item: item_def.clone(),
                                            quantity: 1,
                                            spawn_pos: player_pos,
                                        });
                                    }
                                }
                                ui.end_row();
                            }
                        }
                    });

                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Slots:");
                    ui.label(
                        egui::RichText::new(format!("{items_count}/{items_max}"))
                            .strong()
                            .color(egui::Color32::GOLD),
                    );
                });
                ui.add_space(4.0);
            }));
        } else {
            window.build_func = None;
        }
    }
}

