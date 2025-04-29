use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use sqlx::SqlitePool;

// Import all handlers
use crate::routes::{
    login,
    update_user,
    get_inventory,
    add_product,
    update_product,
    delete_product,
};

pub async fn run() -> std::io::Result<()> {
    let pool = SqlitePool::connect("sqlite:quickstock.db")
        .await
        .expect("Failed to connect to quickstock.db");

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
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(user_pool.clone()))
            .service(login)
            .service(update_user)
            .service(get_inventory)
            .service(add_product)
            .service(update_product)
            .service(delete_product)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
