mod app;
mod inventory;
mod sales;
mod tablereader;
mod settings;
mod ui;
mod popups;
mod widgetdesigners;
mod mainwidget;

fn main() -> Result<(), eframe::Error> {
    app::run()
}
