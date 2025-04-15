use eframe::egui::{self, Window};

pub struct Popup {
    pub open: bool,
    pub message: String,
}

impl Popup {
    pub fn new(message: &str) -> Self {
        Self { open: true, message: message.to_string() }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        if self.open {
            Window::new("Alert")
                .open(&mut self.open)
                .show(ctx, |ui| {
                    ui.label(&self.message);
                    if ui.button("OK").clicked() {
                        self.open = false;
                    }
                });
        }
    }
}
