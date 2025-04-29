use eframe::egui::{Context, Window};

pub struct Popup {
    pub open: bool,
}

impl Popup {
    pub fn new() -> Self {
        Self { open: false }
    }

    pub fn show(&mut self, ctx: &Context) {
        // Only show the window if it's currently marked as open
        if self.open {
            let mut should_close = false;

            Window::new("My Popup")
                .open(&mut self.open)
                .show(ctx, |ui| {
                    ui.label("This is a popup!");

                    if ui.button("Close").clicked() {
                        should_close = true;
                    }
                });

            // Close the popup if the button was clicked
            if should_close {
                self.open = false;
            }
        }
    }
}
