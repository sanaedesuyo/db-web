use axum::extract::{Query, State};
use axum::{Json, Router};
use axum::routing::{delete, get, post};
use sqlx::MySqlPool;
use crate::errors::AppError;
use crate::models::user::*;

pub async fn get_user(
    State(pool): State<MySqlPool>,
    Query(id): Query<u32>
) -> Result<Json<UserDTO>, AppError> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = ?",
        id
    )
        .fetch_one(&pool)
        .await?;

    Ok(Json(user.into()))
}

pub async fn login(
    State(pool): State<MySqlPool>,
    Json(user): Json<LoginUser>,
) -> Result<Json<UserDTO>, AppError> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE name = ? AND password = ?",
        user.name, user.password
    )
        .fetch_one(&pool)
        .await?;

    Ok(Json(user.into()))
}

pub async fn insert_user(
    State(pool): State<MySqlPool>,
    Json(user): Json<InsertUser>,
) -> Result<Json<u64>, AppError> {
    let result = sqlx::query!(
        r#"
        INSERT INTO users (name, password, flag, description)
        VALUES (?, ?, ?, ?)
        "#,
        user.name, user.password, user.flag, user.description
    )
        .execute(&pool)
        .await?;

    Ok(Json(result.rows_affected()))
}

pub async fn update_user(
    State(pool): State<MySqlPool>,
    Json(user): Json<UpdateUser>,
) -> Result<Json<u64>, AppError> {
    let result = sqlx::query!(
        r#"UPDATE users SET
        name = COALESCE(?, name),
        password = COALESCE(?, password),
        flag = COALESCE(?, flag),
        description = COALESCE(?, description)
        WHERE id = ?"#,
        user.name, user.password, user.flag, user.description, user.id
    )
        .execute(&pool)
        .await?;
    
    Ok(Json(result.rows_affected()))
}

pub async fn delete_user(
    State(pool): State<MySqlPool>,
    Query(id): Query<u32>,
) -> Result<Json<u64>, AppError> {
    let result = sqlx::query!(
        "DELETE FROM users WHERE id = ?",
        id
    )
        .execute(&pool)
        .await?;
    
    Ok(Json(result.rows_affected()))
}

pub fn user_routes() -> Router<MySqlPool> {
    Router::new()
        .route("/login", post(login))
        .route("/get", get(get_user))
        .route("/delete", delete(delete_user))
        .route("/add", post(insert_user))
        .route("/update", post(update_user))
}