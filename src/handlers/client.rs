use axum::{Json, extract::State};
use serde_json::Value;
use sqlx::MySqlPool;

use crate::{errors::AppError, middleware::auth::AuthClient, models::client::*, utils::{jwt::{Claims, generate_token}, password::{encrypt_password, verify_password}}};

pub async fn get_client(
    State(pool): State<MySqlPool>,
    AuthClient(auth_client): AuthClient,
) -> Result<Json<Client>, Json<AppError>> {
    let result = sqlx::query_as!(
        Client,
        "SELECT * FROM clients WHERE id = ?",
        auth_client.id,
    )
        .fetch_one(&pool)
        .await
        .map_err(|_| Json(AppError::new("找不到该客户")))?;

    log::info!("{} got self's information", auth_client.username);

    Ok(Json(result))
}

pub async fn insert_client(
    State(pool): State<MySqlPool>,
    Json(client): Json<InsertClient>,
) -> Result<Json<u64>, Json<AppError>> {
    let result = sqlx::query!(
        r#"
        INSERT INTO clients (name, ctype, contactor, contactor_tel, email, description, username, password)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        client.name, client.ctype, client.contactor, client.contactor_tel, client.email, client.description, client.username, encrypt_password(client.password).map_err(|err| {
            log::error!("Failed to encrypt password: {}", err);
            Json(AppError::new("注册时发生错误"))
        })?
    )
        .execute(&pool)
        .await
        .map_err(|_| Json(AppError::new("添加客户失败")))?;

    log::info!("new client '{}' registered.", client.name);

    Ok(Json(result.last_insert_id()))
}

pub async fn update_client(
    State(pool): State<MySqlPool>,
    AuthClient(auth_client): AuthClient,
    Json(client): Json<UpdateClient>,
) -> Result<Json<u64>, Json<AppError>> {
    let mut enc_pwd = None;
    if let Some(pwd) = client.password {
        enc_pwd = Some(encrypt_password(pwd).map_err(|err| {
            log::error!("Failed to encrypt password: {}", err);
            Json(AppError::new("更新时发生错误"))
        })?);
    };

    let result = sqlx::query!(
        r#"UPDATE clients SET
        name = COALESCE(?, name),
        password = COALESCE(?, password),
        contactor = COALESCE(?, contactor),
        contactor_tel = COALESCE(?, contactor_tel),
        email = COALESCE(?, email),
        description = COALESCE(?, description)
        WHERE id = ?"#,
        client.name, enc_pwd, client.contactor, client.contactor_tel, client.email, client.description, client.id
    )
        .execute(&pool)
        .await
        .map_err(|_| Json(AppError::new("更新客户失败")))?;

    log::info!("{} updated", auth_client.username);

    Ok(Json(result.rows_affected()))
}

pub async fn client_login(
    State(pool): State<MySqlPool>,
    Json(param): Json<LoginClient>,
) -> Result<Json<Value>, Json<AppError>> {
    let client = sqlx::query_as!(
        Client,
        "SELECT * FROM clients WHERE username = ?",
        param.username
    )
        .fetch_optional(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("登录时发生错误，请稍后再试"))
        })?;

    let existed_client = client.ok_or_else(|| {
        Json(AppError::new("用户名不存在，请重新输入"))
    })?;

    if verify_password(param.password, &existed_client.password).map_err(|err| {
        log::error!("Failed to verify password: {}", err.to_string());
        Json(AppError::new("登录时发生错误，请稍后再试"))
    })? {
        let claims = Claims::new(
            existed_client.id,
            existed_client.username.clone(),
            existed_client.ctype.clone().into(),
        );

        let token = generate_token(&claims).map_err(|err| {
            log::warn!("Failed to generate token: {}", err);
            AppError::new("生成认证令牌失败")
        })?;

        Ok(Json(serde_json::json!({
            "token": token,
            "client": ClientDTO::from(existed_client),
        })))
    } else {
        Err(Json(AppError::new("密码错误，请重新输入")))
    }
}