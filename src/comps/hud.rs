use egui_macroquad::egui::Ui;

#[derive(Clone)]
pub struct Window {
    pub title: String,
    pub build_func: Option<fn(&mut Ui)>,
}
