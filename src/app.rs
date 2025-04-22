use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use sqlx::SqlitePool;

use crate::routes::{get_inventory, add_product, update_product, delete_product};

pub async fn run() -> std::io::Result<()> {
    let pool = SqlitePool::connect("sqlite:quickstock.db").await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default())
            .app_data(web::Data::new(pool.clone()))
            .service(get_inventory)
            .service(add_product)
            .service(update_product)
            .service(delete_product)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
