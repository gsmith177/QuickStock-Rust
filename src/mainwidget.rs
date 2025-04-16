use eframe::egui;

#[derive(PartialEq)]
enum Page {
    Home,
    Inventory,
    Sales,
    Settings,
}

pub struct MainWidget {
    current_page: Page,
}

impl MainWidget {
    pub fn new() -> Self {
        Self {
            current_page: Page::Home,
        }
    }
}

impl eframe::App for MainWidget {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Home").clicked() {
                    self.current_page = Page::Home;
                }
                if ui.button("Inventory").clicked() {
                    self.current_page = Page::Inventory;
                }
                if ui.button("Sales").clicked() {
                    self.current_page = Page::Sales;
                }
                if ui.button("Settings").clicked() {
                    self.current_page = Page::Settings;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_page {
                Page::Home => {
                    ui.heading("QuickStock Rust");
                    ui.label("Welcome to the app!");
                }
                Page::Inventory => {
                    ui.heading("Inventory Page");
                    ui.label("Here you'll manage your inventory.");
                }
                Page::Sales => {
                    ui.heading("Sales Page");
                    ui.label("Here you'll view sales data.");
                }
                Page::Settings => {
                    ui.heading("Settings Page");
                    ui.label("Here you can configure the app.");
                }
            }
        });
    }
}
