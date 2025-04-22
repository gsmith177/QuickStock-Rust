use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub price: f64,
    pub stock: i32,
}

#[get("/api/inventory")]
async fn get_inventory() -> impl Responder {
    // Replace this with a real DB call eventually
    let products = vec![
        Product { id: 1, name: "F150".to_string(), category: "Truck".to_string(), price: 37450.0, stock: 30 },
        Product { id: 2, name: "RX10".to_string(), category: "Motorcycle".to_string(), price: 6532.0, stock: 34 },
        // etc...
    ];

    HttpResponse::Ok().json(products)
}
