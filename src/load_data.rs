// Import necessary components from rusqlite for SQLite operations
use rusqlite::{params, Connection};

// Import Deserialize trait to enable parsing CSV into structs
use serde::Deserialize;

// For general-purpose error handling
use std::error::Error;

// For handling filesystem paths
use std::path::Path;

/// Represents a product record that can be deserialized from a CSV row
#[derive(Debug, Deserialize)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub quantity: i32,
    pub cost_price: f64,
    pub sell_price: f64,
    pub available: bool,
    pub date_stocked: String,
    pub contact: String,
    pub quantity_sold: i32,
}

/// Loads product data from a CSV file into a SQLite database.
/// If the `products` table does not exist, it is created.
/// Each row from the CSV is inserted into the database, ignoring duplicates.
///
/// # Arguments
/// * `csv_path` - Path to the input CSV file.
/// * `db_path` - Path to the SQLite database file.
///
/// # Returns
/// * `Ok(())` if successful, or a boxed error on failure.
pub fn load_sample_inventory(csv_path: &str, db_path: &str) -> Result<(), Box<dyn Error>> {
    // Open a connection to the SQLite database
    let conn = Connection::open(db_path)?;

    // Create the `products` table if it doesn't already exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            category TEXT NOT NULL,
            quantity INTEGER NOT NULL,
            cost_price REAL NOT NULL,
            sell_price REAL NOT NULL,
            available BOOLEAN NOT NULL,
            date_stocked TEXT NOT NULL,
            contact TEXT NOT NULL,
            quantity_sold INTEGER NOT NULL
        )",
        [], // No parameters needed for table creation
    )?;

    // Initialize a CSV reader from the file path
    let mut rdr = csv::Reader::from_path(Path::new(csv_path))?;

    // Iterate over each row in the CSV
    for result in rdr.deserialize() {
        // Deserialize each row into a `Product` struct
        let product: Product = result?;

        // Insert the product into the database
        // If a product with the same `id` exists, the insertion is ignored
        conn.execute(
            "INSERT OR IGNORE INTO products (
                id, name, category, quantity, cost_price, sell_price, available,
                date_stocked, contact, quantity_sold
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                product.id,
                product.name,
                product.category,
                product.quantity,
                product.cost_price,
                product.sell_price,
                product.available,
                product.date_stocked,
                product.contact,
                product.quantity_sold
            ],
        )?;
    }

    // Confirmation message for successful import
    println!("✅ Sample inventory loaded successfully.");

    // Return success
    Ok(())
}
