use sqlx::PgPool;
use uuid::Uuid;
use crate::models::{User, UserRole};

pub async fn create_user(
    pool: &PgPool,
    id: Uuid,
    email: &str,
    password_hash: &str,
    first_name: &str,
    last_name: &str,
    role: UserRole,
    phone: Option<&str>,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, email, password_hash, first_name, last_name, role, phone)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, email, password_hash, first_name, last_name, 
                  role AS "role: UserRole", phone, is_active, is_verified, 
                  created_at, updated_at
        "#,
        id,
        email,
        password_hash,
        first_name,
        last_name,
        role as UserRole,
        phone
    )
    .fetch_one(pool)
    .await
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, email, password_hash, first_name, last_name, 
               role AS "role: UserRole", phone, is_active, is_verified, 
               created_at, updated_at
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await
}

pub async fn get_user_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, email, password_hash, first_name, last_name, 
               role AS "role: UserRole", phone, is_active, is_verified, 
               created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, email, password_hash, first_name, last_name, 
               role AS "role: UserRole", phone, is_active, is_verified, 
               created_at, updated_at
        FROM users
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
}
