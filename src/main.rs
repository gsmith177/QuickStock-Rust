mod load_data;
mod routes;
mod app;

use std::path::Path;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let csv_path = "data/sample_inventory.csv";
    let db_path = "quickstock.db";

    if Path::new(&db_path).exists() {
        println!("⚙️  Database exists; skipping load.");
    } else if Path::new(&csv_path).exists() {
        if let Err(e) = load_data::load_sample_inventory(csv_path, db_path) {
            eprintln!("❌ Failed to load sample inventory: {}", e);
            std::process::exit(1);
        }
    } else {
        eprintln!("⚠️ No CSV found at {}. Products table not created.", csv_path);
    }

    app::run().await
}
