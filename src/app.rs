use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use sqlx::SqlitePool;

// import your login handler alongside the existing product handlers
use crate::routes::{login, get_inventory, add_product, update_product, delete_product};

pub async fn run() -> std::io::Result<()> {
    // connect to your existing inventory DB
    let pool = SqlitePool::connect("sqlite:quickstock.db")
        .await
        .expect("Failed to connect to quickstock.db");

    // connect to your existing user DB (must be in the same folder as Cargo.toml)
    let user_pool = SqlitePool::connect("sqlite:user.db")
        .await
        .expect("Failed to connect to user.db");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec!["Content-Type"])
                    .supports_credentials()
            )
            // make both pools available to your handlers
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(user_pool.clone()))
            // authentication route
            .service(login)
            // inventory routes
            .service(get_inventory)
            .service(add_product)
            .service(update_product)
            .service(delete_product)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
