// Integration tests for HTTP endpoints using Actix Web's test framework
use actix_web::{test, App};
use quickstock_rust::app::app_config;

#[actix_web::test]
async fn test_health_check_endpoint() {
    // Initialize app with configured routes
    let app = test::init_service(App::new().configure(app_config)).await;

    // Send GET request to /health
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    // Assert HTTP 200 OK
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_products_endpoint() {
    // Initialize app with configured routes
    let app = test::init_service(App::new().configure(app_config)).await;

    // Send GET request to /products
    let req = test::TestRequest::get().uri("/products").to_request();
    let resp = test::call_service(&app, req).await;

    // Assert HTTP 200 OK (even if it returns empty/mocked data)
    assert_eq!(resp.status(), 200);
}
