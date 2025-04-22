mod app;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app::run().await
}
