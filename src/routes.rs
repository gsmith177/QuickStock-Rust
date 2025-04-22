use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Product {
    pub id: Option<i64>,
    pub name: String,
    pub category: String,
    pub price: f64,
    pub stock: i32,
}

#[get("/products")]
pub async fn get_inventory(pool: web::Data<SqlitePool>) -> impl Responder {
    let products = sqlx::query_as::<_, Product>(
        "SELECT id, name, category, price, stock FROM products ORDER BY id"
    )
    .fetch_all(pool.get_ref())
    .await;

    println!("📥 Received request to /products");


    match products {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve products")
        }
    }
}

#[post("/products")]
pub async fn add_product(
    pool: web::Data<SqlitePool>,
    item: web::Json<Product>,
) -> impl Responder {
    let result = sqlx::query(
        "INSERT INTO products (name, category, price, stock) VALUES (?, ?, ?, ?)"
    )
    .bind(&item.name)
    .bind(&item.category)
    .bind(item.price)
    .bind(item.stock)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(rec) => HttpResponse::Ok().json(rec.last_insert_rowid()),
        Err(e) => {
            eprintln!("Insert error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to add product")
        }
    }
}

#[put("/products/{id}")]
pub async fn update_product(
    pool: web::Data<SqlitePool>,
    id: web::Path<i64>,
    item: web::Json<Product>,
) -> impl Responder {
    let result = sqlx::query(
        "UPDATE products SET name = ?, category = ?, price = ?, stock = ? WHERE id = ?"
    )
    .bind(&item.name)
    .bind(&item.category)
    .bind(item.price)
    .bind(item.stock)
    .bind(*id)
    .execute(pool.get_ref())
    .await;
println!("➡️ Adding: {:?}", item);


    match result {
        Ok(_) => HttpResponse::Ok().body("Product updated"),
        Err(e) => {
            eprintln!("Update error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update product")
        }
    }
}

#[delete("/products/{id}")]
pub async fn delete_product(
    pool: web::Data<SqlitePool>,
    id: web::Path<i64>,
) -> impl Responder {
    let result = sqlx::query("DELETE FROM products WHERE id = ?")
        .bind(*id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Product deleted"),
        Err(e) => {
            eprintln!("Delete error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete product")
        }
    }
}
