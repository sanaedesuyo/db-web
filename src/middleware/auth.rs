use axum::{Json, extract::FromRequestParts, http::{StatusCode, header::AUTHORIZATION}};
use chrono::Utc;

use crate::{errors::AppError, models::{client::ClientType, user::UserFlag}, utils::jwt::verify_token};

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: u32,
    pub username: String,
    pub flag: UserFlag
}

#[derive(Debug, Clone)]
pub struct AuthClientS {
    pub id: u32,
    pub username: String,
    pub ctype: ClientType,
}

#[derive(Debug, Clone)]
pub struct AuthClient(pub AuthClientS);

/// 提取当前认证客户的 Extractor
impl<S> FromRequestParts<S> for AuthClient
where
    S: Send + Sync
{
    type Rejection = (StatusCode, Json<AppError>);

    async fn from_request_parts(
            parts: &mut axum::http::request::Parts,
            _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| {
                if s.starts_with("Bearer ") {
                    Some(s[7..].to_string())
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(AppError::new("尊敬的客户，请先登录")),
                )
            })?;

        let claims = verify_token(&token).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(AppError::new("无效令牌")),
            )
        })?;

        if claims.exp < Utc::now().timestamp() {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(AppError::new("登录令牌已过期，请重新登录")),
            ));
        }

        Ok(AuthClient(AuthClientS {
            id: claims.user_id,
            username: claims.username,
            ctype: claims.flag.into(),
        }))
    }
}

/// 提取当前认证用户的 Extractor
impl<S> FromRequestParts<S> for CurrentUser 
where 
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<AppError>);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {

        let token = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|s| {
                if s.starts_with("Bearer ") {
                    Some(s[7..].to_string())
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(AppError::new("缺少认证令牌")),
                )
            })?;

        let claims = verify_token(&token).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(AppError::new("无效令牌")),
            )
        })?;

        if claims.exp < Utc::now().timestamp() {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(AppError::new("登录令牌已过期，请重新登录")),
            ));
        }

        Ok(CurrentUser {
            id: claims.user_id,
            username: claims.username,
            flag: UserFlag::from(claims.flag),
        })
    }
}

/// 要求 Admin 权限的Extractor
#[derive(Debug, Clone)]
pub struct RequireAdmin(pub CurrentUser);

impl<S> FromRequestParts<S> for RequireAdmin 
where 
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<AppError>);

    async fn from_request_parts(
            parts: &mut axum::http::request::Parts,
            state: &S,
        ) -> Result<Self, Self::Rejection> {
        let user = CurrentUser::from_request_parts(parts, state).await?;

        match user.flag {
            UserFlag::Admin => Ok(RequireAdmin(user)),
            _ => Err((
                StatusCode::FORBIDDEN,
                Json(AppError::new("需要管理员权限"))
            )),
        }
    }
}