use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;


use auth::save_credentials;
mod auth;

#[derive(Deserialize)]
struct CredentialUpdate {
    username: String,
    password: String,
}

#[post("/update-credentials")]
async fn update_credentials(form: web::Json<CredentialUpdate>) -> impl Responder {
    match save_credentials(&form.username, &form.password) {
        Ok(_) => HttpResponse::Ok().body("Saved"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed: {}", e)),
    }
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(update_credentials)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
