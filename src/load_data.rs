use rusqlite::{params, Connection};
use serde::Deserialize;
use std::error::Error;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub price: f64,
    pub stock: i32,
}

pub fn load_sample_inventory(csv_path: &str, db_path: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(db_path)?;

    // Create the table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            category TEXT NOT NULL,
            price REAL NOT NULL,
            stock INTEGER NOT NULL
        )",
        [],
    )?;

    // Open and read the CSV
    let mut rdr = csv::Reader::from_path(Path::new(csv_path))?;
    for result in rdr.deserialize() {
        let product: Product = result?;
        conn.execute(
            "INSERT OR IGNORE INTO products (id, name, category, price, stock)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                product.id,
                product.name,
                product.category,
                product.price,
                product.stock
            ],
        )?;
    }

    println!("✅ Sample inventory loaded successfully.");
    Ok(())
}
