use rusqlite::{params, Connection};
use serde::Deserialize;
use std::error::Error;
use std::path::Path;

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

pub fn load_sample_inventory(csv_path: &str, db_path: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(db_path)?;

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
        [],
    )?;

    let mut rdr = csv::Reader::from_path(Path::new(csv_path))?;
    for result in rdr.deserialize() {
        let product: Product = result?;
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

    println!("✅ Sample inventory loaded successfully.");
    Ok(())
}
