use axum::{Json, extract::{Query, State}};
use sqlx::MySqlPool;

use crate::{errors::AppError, middleware::auth::CurrentUser, models::{client::*, user::UserNameQuery}};

pub async fn get_client(
    State(pool): State<MySqlPool>,
    CurrentUser { username, .. }: CurrentUser,
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

    log::info!("{} got client: {}", username, result.name);

    Ok(Json(result))
}

pub async fn insert_client(
    State(pool): State<MySqlPool>,
    CurrentUser { username, .. }: CurrentUser,
    Json(client): Json<InsertClient>,
) -> Result<Json<u64>, Json<AppError>> {
    let result = sqlx::query!(
        r#"
        INSERT INTO clients (name, ctype, contactor, contactor_tel, email, description)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        client.name, client.ctype, client.contactor, client.contactor_tel, client.email, client.description
    )
        .execute(&pool)
        .await
        .map_err(|_| Json(AppError::new("添加客户失败")))?;

    log::info!("{} inserted client: {}", username, client.name);

    Ok(Json(result.last_insert_id()))
}


pub async fn update_client(
    State(pool): State<MySqlPool>,
    CurrentUser { username, .. }: CurrentUser,
    Json(client): Json<UpdateClient>,
) -> Result<Json<u64>, Json<AppError>> {
    let result = sqlx::query!(
        r#"UPDATE clients SET
        name = COALESCE(?, name),
        ctype = COALESCE(?, ctype),
        contactor = COALESCE(?, contactor),
        contactor_tel = COALESCE(?, contactor_tel),
        email = COALESCE(?, email),
        description = COALESCE(?, description)
        WHERE id = ?"#,
        client.name, client.ctype, client.contactor, client.contactor_tel, client.email, client.description, client.id
    )
        .execute(&pool)
        .await
        .map_err(|_| Json(AppError::new("更新客户失败")))?;

    log::info!("{} updated client: {}", username, client.id);

    Ok(Json(result.rows_affected()))
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
