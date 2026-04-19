use egui_macroquad::egui::Ui;

pub struct Window {
    pub title: String,
    pub build_func: Option<Box<dyn Fn(&mut Ui) + Send + Sync + 'static>>,
}
