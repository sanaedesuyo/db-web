use axum::extract::{Query, State};
use axum::Json;
use serde_json::Value;
use sqlx::MySqlPool;
use crate::errors::AppError;
use crate::middleware::auth::RequireAdmin;
use crate::models::user::*;
use crate::utils::jwt::{Claims, generate_token};
use crate::utils::password::{encrypt_password, verify_password};

pub async fn get_user(
    State(pool): State<MySqlPool>,
    RequireAdmin(_admin): RequireAdmin,
    Query(param): Query<UserQueryId>
) -> Result<Json<UserDTO>, Json<AppError>> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = ?",
        param.id
    )
        .fetch_one(&pool)
        .await
        .map_err(|_| Json(AppError::new("该用户不存在")))?;

    Ok(Json(user.into()))
}

pub async fn login(
    State(pool): State<MySqlPool>,
    Json(user): Json<LoginUser>,
) -> Result<Json<Value>, Json<AppError>> {
    let existed_user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE name = ?",
        user.name
    )
        .fetch_optional(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("登录时发生错误"))
        })?
        .ok_or_else(|| {
            Json(AppError::new("用户不存在"))
        })?;

    if verify_password(user.password, &existed_user.password).map_err(|err| {
        log::warn!("{}", err);
        Json(AppError::new("验证密码时发生错误"))
    })? {
        let claims = Claims::new(
            existed_user.id,
            existed_user.name.clone(),
            existed_user.flag.clone().into()
        );

        let token = generate_token(&claims).map_err(|err| {
            log::warn!("Failed to generate token: {}", err);
            Json(AppError::new("生成认证令牌失败"))
        })?;

        Ok(Json(serde_json::json!({
            "user": UserDTO::from(existed_user),
            "token": token,
        })))
    } else {
        Err(Json(AppError::new("密码错误")))
    }
}

pub async fn insert_user(
    State(pool): State<MySqlPool>,
    RequireAdmin(admin): RequireAdmin,
    Json(user): Json<InsertUser>,
) -> Result<Json<u64>, Json<AppError>> {
    let result = sqlx::query!(
        r#"
        INSERT INTO users (name, password, flag, description)
        VALUES (?, ?, ?, ?)
        "#,
        user.name, encrypt_password(user.password).map_err(|err| {
            log::error!("Failed to encrypt password: {}", err);
            Json(AppError::new("注册时发生错误"))
        })?, user.flag, user.description
    )
        .execute(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("添加用户失败"))
        })?;

    log::info!("'{}' created a new '{}' user '{}'", admin.username, String::from(user.flag), user.name);

    Ok(Json(result.last_insert_id()))
}

pub async fn update_user(
    State(pool): State<MySqlPool>,
    RequireAdmin(admin): RequireAdmin,
    Json(user): Json<UpdateUser>,
) -> Result<Json<u64>, Json<AppError>> {
    let mut enc_pwd = None;
    if let Some(pwd) = user.password {
        enc_pwd = Some(encrypt_password(pwd).map_err(|_| {
            Json(AppError::new("更新时发生错误"))
        })?);
    };

    let result = sqlx::query!(
        r#"UPDATE users SET
        name = COALESCE(?, name),
        password = COALESCE(?, password),
        flag = COALESCE(?, flag),
        description = COALESCE(?, description)
        WHERE id = ?"#,
        user.name, enc_pwd, user.flag, user.description, user.id
    )
        .execute(&pool)
        .await
        .map_err(|_| Json(AppError::new("更新用户失败")))?;
    
    log::info!("{} updated info of user {}", admin.username, user.id);
    
    Ok(Json(result.rows_affected()))
}

pub async fn delete_user(
    State(pool): State<MySqlPool>,
    RequireAdmin(admin): RequireAdmin,
    Query(param): Query<UserQueryId>,
) -> Result<Json<u64>, Json<AppError>> {
    let result = sqlx::query!(
        "DELETE FROM users WHERE id = ?",
        param.id
    )
        .execute(&pool)
        .await
        .map_err(|_| Json(AppError::new("删除用户失败")))?;
    
    log::info!("{} deleted user id: {}", admin.username, param.id);
    
    Ok(Json(result.rows_affected()))
}