use axum::{Json, Router, extract::{Query, State}, routing::{get, post}};
use sqlx::MySqlPool;

use crate::{errors::AppError, models::{client::*, user::UserNameQuery}};

pub async fn get_client(
    State(pool): State<MySqlPool>,
    Query(param): Query<ClientQueryId>,
) -> Result<Json<Client>, Json<AppError>> {
    let result = sqlx::query_as!(
        Client,
        "SELECT * FROM clients WHERE id = ?",
        param.id
    )
        .fetch_one(&pool)
        .await
        .map_err(|_| Json(AppError::new("找不到该客户".into())))?;

    log::info!("Got client: {} successfully", result.name);

    Ok(Json(result))
}

pub async fn insert_client(
    State(pool): State<MySqlPool>,
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
        .map_err(|_| Json(AppError::new("添加客户失败".into())))?;

    log::info!("Inserted client: {} successfully", client.name);

    Ok(Json(result.last_insert_id()))
}


pub async fn update_client(
    State(pool): State<MySqlPool>,
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
        .map_err(|_| Json(AppError::new("更新客户失败".into())))?;

    log::info!("Updated client: {} successfully", client.id);

    Ok(Json(result.rows_affected()))
}

pub async fn get_all_clients(
    State(pool): State<MySqlPool>,
) -> Result<Json<Vec<Client>>, Json<AppError>> {
    let result = sqlx::query_as!(
        Client,
        "SELECT * FROM clients"
    )
        .fetch_all(&pool)
        .await
        .map_err(|_| Json(AppError::new("获取客户失败".into())))?;

    log::info!("Got all clients successfully");

    Ok(Json(result))
}

pub async fn get_specified_clients(
    State(pool): State<MySqlPool>,
    Query(param): Query<ClientTypeQuery>
) -> Result<Json<Vec<Client>>, Json<AppError>> {
    let result = sqlx::query_as!(
        Client,
        "SELECT * FROM clients WHERE ctype = ?",
        param.ctype
    )
        .fetch_all(&pool)
        .await
        .map_err(|_| Json(AppError::new("获取客户失败".into())))?;

    log::info!("Got specified clients with type {:?} successfully", param.ctype);

    Ok(Json(result))
}

pub async fn get_clients_by_name_likes(
    State(pool): State<MySqlPool>,
    Query(param): Query<UserNameQuery>,
) -> Result<Json<Vec<Client>>, Json<AppError>> {
    let result = sqlx::query_as!(
        Client,
        "SELECT * FROM clients WHERE name LIKE ?",
        format!("%{}%", param.name)
    )
        .fetch_all(&pool)
        .await
        .map_err(|_| Json(AppError::new("获取客户失败".into())))?;

    log::info!("Got clients by name likes: {} successfully", param.name);

    Ok(Json(result))
}

pub fn client_routes() -> Router<MySqlPool> {
    Router::new()
        .route("/get", get(get_client))
        .route("/add", post(insert_client))
        .route("/update", post(update_client))
        .route("/get_all", get(get_all_clients))
        .route("/get_specified", get(get_specified_clients))
        .route("/get_by_name_likes", get(get_clients_by_name_likes))
}