use eframe::egui;

pub fn label_with_style(ui: &mut egui::Ui, text: &str) {
    ui.label(egui::RichText::new(text).heading());
}
