use actix_web::{HttpRequest, HttpResponse, web};
use std::env;

pub async fn proxy_to_user_service(req: HttpRequest, body: web::Bytes) -> HttpResponse {
    let service_url = env::var("USER_SERVICE_URL").unwrap_or_else(|_| "http://user-service:8001".to_string());
    proxy_request(&service_url, &req, body).await
}

pub async fn proxy_to_payment_service(req: HttpRequest, body: web::Bytes) -> HttpResponse {
    let service_url = env::var("PAYMENT_SERVICE_URL").unwrap_or_else(|_| "http://payment-service:8002".to_string());
    proxy_request(&service_url, &req, body).await
}

pub async fn proxy_to_course_service(req: HttpRequest, body: web::Bytes) -> HttpResponse {
    let service_url = env::var("COURSE_SERVICE_URL").unwrap_or_else(|_| "http://course-service:8003".to_string());
    proxy_request(&service_url, &req, body).await
}

pub async fn proxy_to_ai_service(req: HttpRequest, body: web::Bytes) -> HttpResponse {
    let service_url = env::var("AI_SERVICE_URL").unwrap_or_else(|_| "http://ai-service:8004".to_string());
    proxy_request(&service_url, &req, body).await
}

pub async fn proxy_to_communication_service(req: HttpRequest, body: web::Bytes) -> HttpResponse {
    let service_url = env::var("COMMUNICATION_SERVICE_URL").unwrap_or_else(|_| "http://communication-service:8005".to_string());
    proxy_request(&service_url, &req, body).await
}

async fn proxy_request(service_url: &str, req: &HttpRequest, body: web::Bytes) -> HttpResponse {
    let path = req.uri().path();
    let query = req.uri().query().unwrap_or("");
    let url = if query.is_empty() {
        format!("{}{}", service_url, path)
    } else {
        format!("{}{}?{}", service_url, path, query)
    };

    log::debug!("Proxying request to: {}", url);

    let client = reqwest::Client::new();
    let method = req.method().clone();
    
    let mut request_builder = match method.as_str() {
        "GET" => client.get(&url),
        "POST" => client.post(&url).body(body.to_vec()),
        "PUT" => client.put(&url).body(body.to_vec()),
        "DELETE" => client.delete(&url),
        _ => {
            return HttpResponse::MethodNotAllowed().json(serde_json::json!({
                "error": "Method not allowed"
            }));
        }
    };

    // Forward headers
    for (key, value) in req.headers().iter() {
        if let Ok(val_str) = value.to_str() {
            request_builder = request_builder.header(key.as_str(), val_str);
        }
    }

    match request_builder.send().await {
        Ok(response) => {
            let status = response.status();
            let headers = response.headers().clone();
            
            match response.bytes().await {
                Ok(bytes) => {
                    let mut http_response = HttpResponse::build(
                        actix_web::http::StatusCode::from_u16(status.as_u16())
                            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
                    );
                    
                    // Forward response headers
                    for (key, value) in headers.iter() {
                        if let Ok(val_str) = value.to_str() {
                            http_response.insert_header((key.as_str(), val_str));
                        }
                    }
                    
                    http_response.body(bytes)
                }
                Err(e) => {
                    log::error!("Failed to read response body: {}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to read response from service"
                    }))
                }
            }
        }
        Err(e) => {
            log::error!("Failed to proxy request to {}: {}", url, e);
            HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "error": "Service temporarily unavailable",
                "message": e.to_string()
            }))
        }
    }
}
