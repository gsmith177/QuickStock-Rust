use actix_web::{App, HttpServer, web, middleware::Logger};
use crate::routes::get_inventory;

pub async fn run() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(actix_cors::Cors::permissive()) // Enable CORS
            .service(get_inventory)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
