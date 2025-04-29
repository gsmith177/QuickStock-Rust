use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SaleRecord {
    pub id: i64,
    pub item_id: i64,
    pub quantity_sold: i32,
    pub date: String,
}

pub struct SalesManager;

impl SalesManager {
    // add_sale(), get_sales(), etc.
}
