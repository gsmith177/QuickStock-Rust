use eframe::egui::{self, Context, Window};

pub struct Popup {
    pub open: bool,
}

impl Popup {
    pub fn new() -> Self {
        Self { open: false }
    }

    pub fn show(&mut self, ctx: &Context) {
        let mut is_open = self.open;

        Window::new("My Popup")
            .open(&mut is_open)
            .show(ctx, |ui| {
                ui.label("This is a popup!");

                if ui.button("Close").clicked() {
                    is_open = false;
                }
            });

        // Reflect any changes made inside the popup back to self.open
        self.open = is_open;
    }
}
