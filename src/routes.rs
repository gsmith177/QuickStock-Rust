use crate::db::{InventoryDb, UserDb};
use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use bcrypt::{verify, hash, DEFAULT_COST};

//
// 1) LOGIN HANDLER
//
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(
    db: web::Data<UserDb>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    let pool = &db.0;
    let rec = sqlx::query("SELECT password_hash FROM users WHERE username = ?")
        .bind(&req.username)
        .fetch_one(pool)
        .await;

    if let Ok(row) = rec {
        let hash_str: String = row.try_get("password_hash").unwrap_or_default();
        if verify(&req.password, &hash_str).unwrap_or(false) {
            return HttpResponse::Ok().body("Login successful");
        }
    }
    HttpResponse::Unauthorized().body("Invalid username or password")
}

//
// 2) UPDATE USER HANDLER
//
#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub old_username: String,
    pub new_username: String,
    pub new_password: String,
}

#[put("/update_user")]
pub async fn update_user(
    db: web::Data<UserDb>,
    req: web::Json<UpdateUserRequest>,
) -> impl Responder {
    let pool = &db.0;

    let new_hash = match hash(&req.new_password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => {
            eprintln!("Failed to hash new password");
            return HttpResponse::InternalServerError().body("Hash error");
        }
    };

    let result = sqlx::query(
        "UPDATE users SET username = ?, password_hash = ? WHERE username = ?",
    )
    .bind(&req.new_username)
    .bind(&new_hash)
    .bind(&req.old_username)
    .execute(pool)
    .await;

    match result {
        Ok(r) if r.rows_affected() == 1 => HttpResponse::Ok().body("User updated"),
        Ok(_) => HttpResponse::NotFound().body("User not found"),
        Err(e) => {
            eprintln!("DB error updating user: {:?}", e);
            HttpResponse::InternalServerError().body("Update failed")
        }
    }
}

//
// 3) PRODUCT HANDLERS
//
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Product {
    pub id: Option<i64>,
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

#[get("/products")]
pub async fn get_inventory(
    db: web::Data<InventoryDb>,
) -> impl Responder {
    let pool = &db.0;
    let products = sqlx::query_as::<_, Product>(
        "SELECT id, name, category, quantity, cost_price, sell_price, available, \
         date_stocked, contact, quantity_sold \
         FROM products ORDER BY id"
    )
    .fetch_all(pool)
    .await;

    match products {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e)  => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve products")
        }
    }
}

#[post("/products")]
pub async fn add_product(
    db: web::Data<InventoryDb>,
    item: web::Json<Product>,
) -> impl Responder {
    let pool = &db.0;
    let result = sqlx::query(
        "INSERT INTO products \
         (name, category, quantity, cost_price, sell_price, available, date_stocked, contact, quantity_sold) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&item.name)
    .bind(&item.category)
    .bind(item.quantity)
    .bind(item.cost_price)
    .bind(item.sell_price)
    .bind(item.available)
    .bind(&item.date_stocked)
    .bind(&item.contact)
    .bind(item.quantity_sold)
    .execute(pool)
    .await;

    match result {
        Ok(rec) => HttpResponse::Ok().json(rec.last_insert_rowid()),
        Err(e)  => {
            eprintln!("Insert error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to add product")
        }
    }
}

#[put("/products/{id}")]
pub async fn update_product(
    db: web::Data<InventoryDb>,
    id: web::Path<i64>,
    item: web::Json<Product>,
) -> impl Responder {
    let pool = &db.0;
    let result = sqlx::query(
        "UPDATE products SET \
         name = ?, category = ?, quantity = ?, cost_price = ?, sell_price = ?, \
         available = ?, date_stocked = ?, contact = ?, quantity_sold = ? \
         WHERE id = ?"
    )
    .bind(&item.name)
    .bind(&item.category)
    .bind(item.quantity)
    .bind(item.cost_price)
    .bind(item.sell_price)
    .bind(item.available)
    .bind(&item.date_stocked)
    .bind(&item.contact)
    .bind(item.quantity_sold)
    .bind(*id)
    .execute(pool)
    .await;

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
    db: web::Data<InventoryDb>,
    id: web::Path<i64>,
) -> impl Responder {
    let pool = &db.0;
    let result = sqlx::query("DELETE FROM products WHERE id = ?")
        .bind(*id)
        .execute(pool)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Product deleted"),
        Err(e) => {
            eprintln!("Delete error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete product")
        }
    }
}
