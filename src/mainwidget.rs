use eframe::egui;

pub struct MainWidget {}

impl MainWidget {
    pub fn new() -> Self {
        Self {}
    }
}

impl eframe::App for MainWidget {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("QuickStock Rust");
            ui.label("Welcome to the app!");
        });
    }
}
