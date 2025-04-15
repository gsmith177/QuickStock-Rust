use crate::mainwidget::MainWidget;
use eframe::NativeOptions;

pub fn run() -> Result<(), eframe::Error> {
    let options = NativeOptions::default();
    eframe::run_native(
        "QuickStock Rust",
        options,
        Box::new(|_cc| Box::new(MainWidget::new())),
    )
}
