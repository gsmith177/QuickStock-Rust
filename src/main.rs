// Declare internal modules for organization
mod load_data; // Handles loading inventory from CSV into SQLite
mod routes;    // Defines all HTTP route handlers
mod app;       // Contains server configuration and setup
mod db;        // Defines database wrappers and logic

// Suppresses warnings about unused functions or modules (useful during testing)
#[allow(dead_code)]

use std::path::Path; // Used to check file existence

/// Entry point for the Actix web server application
#[actix_web::main] // Indicates that this is an asynchronous Actix runtime entry
async fn main() -> std::io::Result<()> {
    // Define paths to the CSV file and SQLite database
    let csv_path = "data/sample_inventory.csv";
    let db_path = "quickstock.db";

    // If the database already exists, skip loading sample inventory
    if Path::new(&db_path).exists() {
        println!("⚙️  Database exists; skipping load.");
    }
    // If the CSV exists but no DB, attempt to load inventory into a new database
    else if Path::new(&csv_path).exists() {
        if let Err(e) = load_data::load_sample_inventory(csv_path, db_path) {
            // Print an error and exit if loading fails
            eprintln!("❌ Failed to load sample inventory: {}", e);
            std::process::exit(1);
        }
    }
    // If neither the database nor CSV exists, warn the user
    else {
        eprintln!("⚠️ No CSV found at {}. Products table not created.", csv_path);
    }

    // Start the Actix web server (configured in app.rs)
    app::run().await
}
