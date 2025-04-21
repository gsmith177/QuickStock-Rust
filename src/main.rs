mod app;
mod load_data;

use std::path::Path;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load data
    let csv_path = "data/sample_inventory.csv";
    let db_path = "quickstock.db";

    if Path::new(csv_path).exists() {
        if let Err(e) = load_data::load_sample_inventory(csv_path, db_path) {
            eprintln!("❌ Failed to load sample inventory: {}", e);
        }
    } else {
        eprintln!("⚠️ CSV file not found at path: {}", csv_path);
    }

    // Start the app
    app::run().await
}
