use actix_web::{web, App, HttpResponse, HttpServer, middleware};
use actix_cors::Cors;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = env::var("PORT").unwrap_or_else(|_| "8004".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let bind_address = format!("{}:{}", host, port);

    log::info!("🚀 Starting ai-service on {}", bind_address);

    HttpServer::new(|| {
        let cors = Cors::permissive();
        
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .route("/health", web::get().to(health_check))
            .route("/", web::get().to(index))
    })
    .bind(&bind_address)?
    .run()
    .await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "ai-service",
        "version": "1.0.0",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "service": "ai-service",
        "version": "1.0.0",
        "status": "running"
    }))
}
