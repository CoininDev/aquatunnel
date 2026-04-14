use egui_macroquad::egui::{self, ahash::{HashMap, HashMapExt}};
use legion::{world::SubWorld, *};
use crate::{
    comps::Window, resources::input::InputContext,
};

#[system]
#[write_component(Window)]
pub fn load_windows(
    world: &mut SubWorld,
    #[resource] input: &mut InputContext,
) {
    let mut builds: HashMap<String, fn(&mut egui::Ui)> = HashMap::new();
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

        for (t, b) in builds.clone().into_iter() {
            egui::Window::new(t).show(ctx, b);
        }
    });

    for w in windows {
        if let Some(f) = builds.get(&w.title) {
            w.build_func = Some(*f);
        }
    }
}

#[system]
pub fn render_egui(
) {
    egui_macroquad::draw();
}
