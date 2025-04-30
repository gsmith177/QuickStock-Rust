// Suppress dead code warnings during testing (useful when functions are only used in test configs)
#![cfg_attr(test, allow(dead_code))]

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use sqlx::SqlitePool;

// Import type wrappers for SQLite pools
use crate::db::{InventoryDb, UserDb};

// Import all route handlers used in the application
use crate::routes::{
    login,
    update_user,
    get_inventory,
    add_product,
    update_product,
    delete_product,
};

/// Launches the Actix-web server with route handlers and database pools configured
pub async fn run() -> std::io::Result<()> {
    // Create connection pool to main inventory database
    let pool = SqlitePool::connect("sqlite:quickstock.db")
        .await
        .expect("Failed to connect to quickstock.db");

    // Create connection pool to user authentication database
    let user_pool = SqlitePool::connect("sqlite:user.db")
        .await
        .expect("Failed to connect to user.db");
    
    // Start HTTP server on localhost:8080
    HttpServer::new(move || {
        App::new()
            // Apply CORS policy to allow requests from React frontend
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec!["Content-Type"])
                    .supports_credentials()
            )
            // Inject the database connection pools into app state
            .app_data(web::Data::new(InventoryDb(pool.clone())))
            .app_data(web::Data::new(UserDb(user_pool.clone())))
            // Register all main API routes
            .service(login)
            .service(update_user)
            .service(get_inventory)
            .service(add_product)
            .service(update_product)
            .service(delete_product)
    })
    .bind(("127.0.0.1", 8080))?  // Bind to local address and port
    .run()
    .await
}

// === Testing-only config section ===

use crate::routes::{health_check, list_products};

/// Provides route config for testing only (e.g., for use in integration tests)
#[allow(dead_code)]
pub fn app_config(cfg: &mut web::ServiceConfig) {
    // Add health check route
    cfg.service(web::resource("/health").route(web::get().to(health_check)));

    // Add dummy products listing route
    cfg.service(web::resource("/products").route(web::get().to(list_products)));
}
