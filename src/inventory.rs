use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct InventoryItem {
    pub id: i64,
    pub name: String,
    pub quantity: i32,
}

pub struct Inventory {
    // maybe include pool: SqlitePool later
}

impl Inventory {
    // methods like add_item, remove_item, etc.
}
