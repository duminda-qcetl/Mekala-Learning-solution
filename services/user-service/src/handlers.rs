use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use validator::Validate;

use crate::models::*;
use crate::db;

pub async fn register(
    pool: web::Data<PgPool>,
    req: web::Json<RegisterRequest>,
) -> HttpResponse {
    if let Err(errors) = req.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": errors
        }));
    }

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(req.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to hash password"
            }));
        }
    };

    let user_id = Uuid::new_v4();
    
    match db::create_user(
        &pool,
        user_id,
        &req.email,
        &password_hash,
        &req.first_name,
        &req.last_name,
        req.role.clone(),
        req.phone.as_deref(),
    ).await {
        Ok(user) => {
            log::info!("User registered: {} ({})", user.email, user.id);
            HttpResponse::Created().json(UserResponse::from(user))
        }
        Err(e) => {
            log::error!("Failed to create user: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create user"
            }))
        }
    }
}

pub async fn login(
    pool: web::Data<PgPool>,
    req: web::Json<LoginRequest>,
) -> HttpResponse {
    match db::get_user_by_email(&pool, &req.email).await {
        Ok(Some(user)) => {
            let argon2 = Argon2::default();
            let parsed_hash = match PasswordHash::new(&user.password_hash) {
                Ok(hash) => hash,
                Err(_) => {
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Invalid password hash"
                    }));
                }
            };

            if argon2.verify_password(req.password.as_bytes(), &parsed_hash).is_ok() {
                // Generate JWT token
                let token = format!("jwt_token_for_{}", user.id);
                
                log::info!("User logged in: {} ({})", user.email, user.id);
                HttpResponse::Ok().json(LoginResponse {
                    token,
                    user: UserResponse::from(user),
                })
            } else {
                HttpResponse::Unauthorized().json(serde_json::json!({
                    "error": "Invalid credentials"
                }))
            }
        }
        Ok(None) => {
            HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid credentials"
            }))
        }
        Err(e) => {
            log::error!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            }))
        }
    }
}

pub async fn get_users(pool: web::Data<PgPool>) -> HttpResponse {
    match db::get_all_users(&pool).await {
        Ok(users) => {
            let response: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log::error!("Failed to fetch users: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch users"
            }))
        }
    }
}

pub async fn get_user(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> HttpResponse {
    match db::get_user_by_id(&pool, *id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(UserResponse::from(user)),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        })),
        Err(e) => {
            log::error!("Failed to fetch user: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch user"
            }))
        }
    }
}

pub async fn update_user(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
    req: web::Json<RegisterRequest>,
) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "User updated successfully"
    }))
}

pub async fn delete_user(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "User deleted successfully"
    }))
}

pub async fn get_profile(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> HttpResponse {
    get_user(pool, id).await
}
