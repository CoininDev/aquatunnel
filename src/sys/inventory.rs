
use egui_macroquad::egui;
use legion::{world::SubWorld, *};
use crate::{comps::*, resources::input::{InputAction, InputContext}};

#[system]
#[read_component(InventoryHolder)]
#[write_component(Window)]
pub fn inventory_window(
    #[state] open: &mut bool,
    #[state] cooldown: &mut i32,
    #[resource] input: &mut InputContext,
    world: &mut SubWorld
) {
    *cooldown -= 1;
    if *cooldown <= 0 {
        *open = *open ^ input.consume_action(InputAction::InventoryToggle);
        *cooldown = 4;
    }

    let holder = <(&InventoryHolder, &mut Window)>::query()
        .iter_mut(world)
        .next();

    if let Some((holder, window)) = holder {
        if *open {
            let items = holder.inventory.items.clone();
            let items_count = holder
                .inventory.items
                .iter().filter(|a| a.item.is_some())
                .collect::<Vec<_>>().len();
            let items_max = holder.inventory.max;
            window.build_func = Some(Box::new(move |ui| {
                ui.label(egui::RichText::new("Itens").strong().size(16.0));
                ui.separator();

                egui::Grid::new("inventory_grid")
                    .num_columns(2)
                    .spacing([40.0, 6.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Usar").underline());
                        ui.label(egui::RichText::new("Item").underline());
                        ui.label(egui::RichText::new("Qtd.").underline());
                        ui.end_row();

                        for item in items.clone() {
                            if let Some(item) = item.item {
                                if ui.button("Usar").clicked() { println!("Usando {}", item.name); }
                                ui.label(item.name);
                                ui.label("1");
                                ui.end_row();
                            }
                        }
                    });

                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Total de itens:");
                    ui.label(egui::RichText::new(items_count.to_string())
                        .strong()
                        .color(egui::Color32::GOLD));
                    ui.label(format!("/{items_max}"));
                });

                ui.add_space(4.0);
            }));
        } else {
            window.build_func = None;
        }
    } 
}
