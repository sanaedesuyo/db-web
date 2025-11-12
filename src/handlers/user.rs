use axum::extract::{Query, State};
use axum::{Json, Router};
use axum::routing::{delete, get, post};
use serde::Deserialize;
use sqlx::MySqlPool;
use crate::errors::AppError;
use crate::models::user::*;

#[derive(Debug, Deserialize)]
pub struct UserQueryId {
    pub id: u32,
}

pub async fn get_user(
    State(pool): State<MySqlPool>,
    Query(param): Query<UserQueryId>
) -> Result<Json<UserDTO>, Json<AppError>> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = ?",
        param.id
    )
        .fetch_one(&pool)
        .await
        .map_err(|_| AppError::new("该用户不存在".into()))?;

    Ok(Json(user.into()))
}

pub async fn login(
    State(pool): State<MySqlPool>,
    Json(user): Json<LoginUser>,
) -> Result<Json<UserDTO>, Json<AppError>> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE name = ? AND password = ?",
        user.name, user.password
    )
        .fetch_one(&pool)
        .await
        .map_err(|_| AppError::new("用户名或密码错误".into()))?;

    Ok(Json(user.into()))
}

pub async fn insert_user(
    State(pool): State<MySqlPool>,
    Json(user): Json<InsertUser>,
) -> Result<Json<u64>, Json<AppError>> {
    let result = sqlx::query!(
        r#"
        INSERT INTO users (name, password, flag, description)
        VALUES (?, ?, ?, ?)
        "#,
        user.name, user.password, user.flag, user.description
    )
        .execute(&pool)
        .await
        .map_err(|_| AppError::new("添加用户失败".into()))?;

    Ok(Json(result.rows_affected()))
}

pub async fn update_user(
    State(pool): State<MySqlPool>,
    Json(user): Json<UpdateUser>,
) -> Result<Json<u64>, Json<AppError>> {
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
        .await
        .map_err(|_| AppError::new("更新用户失败".into()))?;
    
    Ok(Json(result.rows_affected()))
}

pub async fn delete_user(
    State(pool): State<MySqlPool>,
    Query(param): Query<UserQueryId>,
) -> Result<Json<u64>, Json<AppError>> {
    let result = sqlx::query!(
        "DELETE FROM users WHERE id = ?",
        param.id
    )
        .execute(&pool)
        .await
        .map_err(|_| AppError::new("删除用户失败".into()))?;
    
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