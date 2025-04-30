// Allow unused code during testing to prevent warnings
#![cfg_attr(test, allow(dead_code))]

// Import database wrappers for user and inventory operations
use crate::db::{InventoryDb, UserDb};

// Import Actix Web components for routing and HTTP responses
use actix_web::{get, post, put, delete, web, Responder, HttpResponse};

// For serializing/deserializing request/response payloads
use serde::{Deserialize, Serialize};

// For extracting column values from SQL rows
use sqlx::Row;

// For securely hashing and verifying passwords
use bcrypt::{verify, hash, DEFAULT_COST};

//
// 1) LOGIN HANDLER
//

/// Struct to deserialize login request JSON payload
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Handles POST /login route: verifies user credentials against the database
#[post("/login")]
pub async fn login(
    db: web::Data<UserDb>,         // User database pool
    req: web::Json<LoginRequest>, // Incoming login request JSON
) -> impl Responder {
    let pool = &db.0;

    // Attempt to retrieve the password hash for the given username
    let rec = sqlx::query("SELECT password_hash FROM users WHERE username = ?")
        .bind(&req.username)
        .fetch_one(pool)
        .await;

    // If user found, compare provided password with stored hash
    if let Ok(row) = rec {
        let hash_str: String = row.try_get("password_hash").unwrap_or_default();
        if verify(&req.password, &hash_str).unwrap_or(false) {
            return HttpResponse::Ok().body("Login successful");
        }
    }

    // If not found or password mismatch, return unauthorized
    HttpResponse::Unauthorized().body("Invalid username or password")
}

//
// 2) UPDATE USER HANDLER
//

/// Struct to handle updating an existing user's username and password
#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub old_username: String,
    pub new_username: String,
    pub new_password: String,
}

/// Handles PUT /update_user route: updates credentials for an existing user
#[put("/update_user")]
pub async fn update_user(
    db: web::Data<UserDb>,
    req: web::Json<UpdateUserRequest>,
) -> impl Responder {
    let pool = &db.0;

    // Hash the new password securely
    let new_hash = match hash(&req.new_password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => {
            eprintln!("Failed to hash new password");
            return HttpResponse::InternalServerError().body("Hash error");
        }
    };

    // Execute the update query
    let result = sqlx::query(
        "UPDATE users SET username = ?, password_hash = ? WHERE username = ?",
    )
    .bind(&req.new_username)
    .bind(&new_hash)
    .bind(&req.old_username)
    .execute(pool)
    .await;

    // Return appropriate response based on result
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

/// Struct representing a product entry (used for both API and DB operations)
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

/// Handles GET /products route: fetches all products from the database
#[get("/products")]
pub async fn get_inventory(
    db: web::Data<InventoryDb>,
) -> impl Responder {
    let pool = &db.0;

    // Fetch all product rows into Product structs
    let products = sqlx::query_as::<_, Product>(
        "SELECT id, name, category, quantity, cost_price, sell_price, available, \
         date_stocked, contact, quantity_sold \
         FROM products ORDER BY id"
    )
    .fetch_all(pool)
    .await;

    // Respond with product list or error
    match products {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e)  => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve products")
        }
    }
}

/// Handles POST /products route: adds a new product to the database
#[post("/products")]
pub async fn add_product(
    db: web::Data<InventoryDb>,
    item: web::Json<Product>,
) -> impl Responder {
    let pool = &db.0;

    // Insert new product
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

    // Respond with new row ID or error
    match result {
        Ok(rec) => HttpResponse::Ok().json(rec.last_insert_rowid()),
        Err(e)  => {
            eprintln!("Insert error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to add product")
        }
    }
}

/// Handles PUT /products/{id} route: updates a product by ID
#[put("/products/{id}")]
pub async fn update_product(
    db: web::Data<InventoryDb>,
    id: web::Path<i64>,
    item: web::Json<Product>,
) -> impl Responder {
    let pool = &db.0;

    // Execute update query
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

    // Respond with status
    match result {
        Ok(_) => HttpResponse::Ok().body("Product updated"),
        Err(e) => {
            eprintln!("Update error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update product")
        }
    }
}

/// Handles DELETE /products/{id} route: deletes a product by ID
#[delete("/products/{id}")]
pub async fn delete_product(
    db: web::Data<InventoryDb>,
    id: web::Path<i64>,
) -> impl Responder {
    let pool = &db.0;

    // Execute deletion query
    let result = sqlx::query("DELETE FROM products WHERE id = ?")
        .bind(*id)
        .execute(pool)
        .await;

    // Return appropriate response
    match result {
        Ok(_) => HttpResponse::Ok().body("Product deleted"),
        Err(e) => {
            eprintln!("Delete error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete product")
        }
    }
}

//
// Test Coverage Endpoints
//

/// Basic health check route used in integration tests
#[allow(dead_code)]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

/// Dummy products list route used in tests
#[allow(dead_code)]
pub async fn list_products() -> impl Responder {
    HttpResponse::Ok().body("List of products (mock)")
}
