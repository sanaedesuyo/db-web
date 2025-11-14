use axum::{Json, Router, extract::{Query, State}, routing::{delete, get, post}};
use sqlx::MySqlPool;
use crate::{errors::AppError, models::repository::*};

pub async fn get_repository(
    State(pool): State<MySqlPool>,
    Query(param): Query<RepositoryQueryId>,
) -> Result<Json<Repository>, Json<AppError>> {
    let result = sqlx::query_as!(
        Repository,
        "SELECT * FROM repository WHERE id = ?",
        param.id
    )
        .fetch_one(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("找不到该仓库".into()))
        })?;

    log::info!("Got repository: {} successfully", result.name);

    Ok(Json(result))
}

pub async fn insert_repository(
    State(pool): State<MySqlPool>,
    Json(repository): Json<InsertRepository>,
) -> Result<Json<u64>, Json<AppError>> {
    let result = sqlx::query!(
        r#"INSERT INTO repository (name, description)
        VALUES (?, ?)
        "#,
        repository.name, repository.description
    )
        .execute(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("添加仓库失败".into()))
        })?;

    log::info!("Inserted repository: {} successfully", repository.name);

    Ok(Json(result.last_insert_id()))
}

pub async fn update_repository(
    State(pool): State<MySqlPool>,
    Json(repository): Json<UpdateRepository>,
) -> Result<Json<u64>, Json<AppError>> {
    let result = sqlx::query!(
        r#"UPDATE repository SET name = ?, description = ?
        WHERE id = ?
        "#,
        repository.name, repository.description, repository.id
    )
        .execute(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("更新仓库失败".into()))
        })?;

    Ok(Json(result.rows_affected()))
}

pub async fn get_all_repositories(
    State(pool): State<MySqlPool>,
) -> Result<Json<Vec<Repository>>, Json<AppError>> {
    let result = sqlx::query_as!(
        Repository,
        "SELECT * FROM repository",
    )
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("无法获取所有仓库".into()))
        })?;

    Ok(Json(result))
}

pub async fn get_repository_by_name_likes(
    State(pool): State<MySqlPool>,
    Query(param): Query<RepositoryNameQuery>,
) -> Result<Json<Vec<Repository>>, Json<AppError>> {
    let result = sqlx::query_as!(
        Repository,
        "SELECT * FROM repository WHERE name LIKE ?",
        format!("%{}%", param.name)
    )
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("无法获取仓库".into()))
        })?;

    Ok(Json(result))
}

pub async fn delete_repository(
    State(pool): State<MySqlPool>,
    Query(param): Query<RepositoryQueryId>,
) -> Result<Json<u64>, Json<AppError>> {
    let result = sqlx::query!(
        "DELETE FROM repository WHERE id = ?",
        param.id
    )
        .execute(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("删除仓库失败".into()))
        })?;

    Ok(Json(result.rows_affected()))
}

pub fn repository_routes() -> Router<MySqlPool> {
    Router::new()
        .route("/get", get(get_repository))
        .route("/add", post(insert_repository))
        .route("/update", post(update_repository))
        .route("/get_all", get(get_all_repositories))
        .route("/get_by_name_likes", get(get_repository_by_name_likes))
        .route("/delete", delete(delete_repository))
}