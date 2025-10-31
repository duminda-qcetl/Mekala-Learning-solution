use actix_web::{web, App, HttpResponse, HttpServer, middleware};
use actix_cors::Cors;
use std::env;

mod auth;
mod proxy;
mod middleware as custom_middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let bind_address = format!("{}:{}", host, port);

    log::info!("🚀 Starting Mekala API Gateway on {}", bind_address);
    log::info!("📡 Service endpoints:");
    log::info!("   - User Service: {}", env::var("USER_SERVICE_URL").unwrap_or_else(|_| "http://user-service:8001".to_string()));
    log::info!("   - Payment Service: {}", env::var("PAYMENT_SERVICE_URL").unwrap_or_else(|_| "http://payment-service:8002".to_string()));
    log::info!("   - Course Service: {}", env::var("COURSE_SERVICE_URL").unwrap_or_else(|_| "http://course-service:8003".to_string()));
    log::info!("   - AI Service: {}", env::var("AI_SERVICE_URL").unwrap_or_else(|_| "http://ai-service:8004".to_string()));
    log::info!("   - Communication Service: {}", env::var("COMMUNICATION_SERVICE_URL").unwrap_or_else(|_| "http://communication-service:8005".to_string()));

    HttpServer::new(|| {
        let cors = Cors::permissive();
        
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .route("/health", web::get().to(health_check))
            .route("/", web::get().to(index))
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/users")
                            .route("", web::get().to(proxy::proxy_to_user_service))
                            .route("", web::post().to(proxy::proxy_to_user_service))
                            .route("/{id}", web::get().to(proxy::proxy_to_user_service))
                            .route("/{id}", web::put().to(proxy::proxy_to_user_service))
                            .route("/{id}", web::delete().to(proxy::proxy_to_user_service))
                            .route("/login", web::post().to(proxy::proxy_to_user_service))
                            .route("/register", web::post().to(proxy::proxy_to_user_service))
                    )
                    .service(
                        web::scope("/payments")
                            .route("", web::get().to(proxy::proxy_to_payment_service))
                            .route("", web::post().to(proxy::proxy_to_payment_service))
                            .route("/{id}", web::get().to(proxy::proxy_to_payment_service))
                    )
                    .service(
                        web::scope("/courses")
                            .route("", web::get().to(proxy::proxy_to_course_service))
                            .route("", web::post().to(proxy::proxy_to_course_service))
                            .route("/{id}", web::get().to(proxy::proxy_to_course_service))
                            .route("/{id}", web::put().to(proxy::proxy_to_course_service))
                    )
                    .service(
                        web::scope("/ai")
                            .route("/handwriting", web::post().to(proxy::proxy_to_ai_service))
                            .route("/pronunciation", web::post().to(proxy::proxy_to_ai_service))
                            .route("/speech-to-text", web::post().to(proxy::proxy_to_ai_service))
                    )
                    .service(
                        web::scope("/communication")
                            .route("/chat", web::post().to(proxy::proxy_to_communication_service))
                            .route("/notifications", web::post().to(proxy::proxy_to_communication_service))
                    )
            )
    })
    .bind(&bind_address)?
    .run()
    .await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "api-gateway",
        "version": "1.0.0",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "name": "Mekala Smart Language Learning Platform",
        "version": "1.0.0",
        "description": "AI-powered language learning ecosystem",
        "endpoints": {
            "health": "/health",
            "users": "/api/v1/users",
            "payments": "/api/v1/payments",
            "courses": "/api/v1/courses",
            "ai": "/api/v1/ai",
            "communication": "/api/v1/communication"
        }
    }))
}
