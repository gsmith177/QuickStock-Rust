use eframe::egui::{self, Ui};

pub fn create_button(ui: &mut Ui, label: &str) -> egui::Response {
    ui.button(label)
}
