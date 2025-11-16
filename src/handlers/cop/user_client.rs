use axum::{Json, extract::{Query, State}};
use sqlx::MySqlPool;

use crate::{errors::AppError, middleware::auth::CurrentUser, models::{client::{Client, ClientQueryId, ClientTypeModifyQuery, ClientTypeQuery}, user::UserNameQuery}};


pub async fn get_clients_by_name_likes(
    State(pool): State<MySqlPool>,
    CurrentUser { username, .. }: CurrentUser,
    Query(param): Query<UserNameQuery>,
) -> Result<Json<Vec<Client>>, Json<AppError>> {
    let result = sqlx::query_as!(
        Client,
        "SELECT * FROM clients WHERE name LIKE ?",
        format!("%{}%", param.name)
    )
        .fetch_all(&pool)
        .await
        .map_err(|_| Json(AppError::new("获取客户失败")))?;

    log::info!("{} got clients by name likes: {} successfully", username, param.name);

    Ok(Json(result))
}

pub async fn get_specified_clients(
    State(pool): State<MySqlPool>,
    CurrentUser { username, .. }: CurrentUser,
    Query(param): Query<ClientTypeQuery>
) -> Result<Json<Vec<Client>>, Json<AppError>> {
    let result = sqlx::query_as!(
        Client,
        "SELECT * FROM clients WHERE ctype = ?",
        param.ctype
    )
        .fetch_all(&pool)
        .await
        .map_err(|_| Json(AppError::new("获取客户失败")))?;

    log::info!("{} got specified clients with type {:?} successfully", username, param.ctype);

    Ok(Json(result))
}

pub async fn get_all_clients(
    State(pool): State<MySqlPool>,
    CurrentUser { username, .. }: CurrentUser,
) -> Result<Json<Vec<Client>>, Json<AppError>> {
    let result = sqlx::query_as!(
        Client,
        "SELECT * FROM clients"
    )
        .fetch_all(&pool)
        .await
        .map_err(|_| Json(AppError::new("获取客户失败")))?;

    log::info!("{} got all clients", username);

    Ok(Json(result))
}

pub async fn modify_client_type(
    State(pool): State<MySqlPool>,
    CurrentUser {username, ..}: CurrentUser,
    Query(param): Query<ClientTypeModifyQuery>,
) -> Result<Json<u64>, Json<AppError>> {
    let result = sqlx::query!(
        r#"UPDATE clients SET
        ctype = ?
        WHERE id = ?"#,
        param.ctype, param.id
    )
        .execute(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("更新失败"))
        })?;

    log::info!("{} modified type of client id {}", username, param.id);
    
    Ok(Json(result.rows_affected()))
}

pub async fn user_get_client(
    State(pool): State<MySqlPool>,
    CurrentUser {username, ..}: CurrentUser,
    Query(param): Query<ClientQueryId>,
) -> Result<Json<Client>, Json<AppError>> {
    let result = sqlx::query_as!(
        Client,
        "SELECT * FROM clients WHERE id = ?",
        param.id
    )
        .fetch_one(&pool)
        .await
        .map_err(|_| Json(AppError::new("找不到该客户")))?;

    log::info!("{} got {} client's information", username, param.id);

    Ok(Json(result))
}