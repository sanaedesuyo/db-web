use axum::{Json, extract::{Query, State}};
use sqlx::MySqlPool;
use crate::{errors::AppError, middleware::auth::CurrentUser, models::{page::{PageQuery, PageResponse}, product::*}, utils::page_query::no_conditional_page_query};

pub async fn get_product(
    State(pool): State<MySqlPool>,
    CurrentUser { username, .. }: CurrentUser,
    Query(param): Query<ProductQueryId>,
) -> Result<Json<Product>, Json<AppError>> {
    let result = sqlx::query_as!(
        Product,
        "SELECT * FROM products WHERE id = ?",
        param.id
    )
        .fetch_one(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("找不到该产品"))
        })?;

    log::info!("{} got product info of {}", username, result.name);

    Ok(Json(result))
}

pub async fn insert_product(
    State(pool): State<MySqlPool>,
    CurrentUser { username, .. }: CurrentUser,
    Json(product): Json<InsertProduct>,
) -> Result<Json<u64>, Json<AppError>> {
    let result = sqlx::query!(
        r#"INSERT INTO products
        (name, size, price, max_amount, min_amount)
        VALUES (?, ?, ?, ?, ?)
        "#,
        product.name, product.size, product.price, product.max_amount, product.min_amount
    )
        .execute(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("添加产品失败"))
        })?;

    log::info!("{} inserted new product {}", username, product.name);

    Ok(Json(result.last_insert_id()))
}

pub async fn update_product(
    State(pool): State<MySqlPool>,
    CurrentUser { username, .. }: CurrentUser,
    Json(product): Json<UpdateProduct>,
) -> Result<Json<u64>, Json<AppError>> {
    let result = sqlx::query!(
        r#"UPDATE products SET
        name = COALESCE(?, name),
        size = COALESCE(?, size),
        price = COALESCE(?, price),
        max_amount = COALESCE(?, max_amount),
        min_amount = COALESCE(?, min_amount)
        WHERE id = ?"#,
        product.name, product.size, product.price, product.max_amount, product.min_amount, product.id
    )
        .execute(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("更新产品信息失败"))
        })?;

    log::info!("{} updated product info with id: {}", username, product.id);

    Ok(Json(result.rows_affected()))
}

pub async fn get_all_product(
    State(pool): State<MySqlPool>,
    CurrentUser { username, .. }: CurrentUser,
) -> Result<Json<Vec<Product>>, Json<AppError>> {
    let result = sqlx::query_as!(
        Product,
        "SELECT * FROM products",
    )
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("无法获取产品信息列表"))
        })?;

    log::info!("{} got all product info", username);

    Ok(Json(result))
}

pub async fn get_product_page(
    State(pool): State<MySqlPool>,
    CurrentUser { username, .. }: CurrentUser,
    Query(page_query): Query<PageQuery>,
) -> Result<Json<PageResponse<Product>>, Json<AppError>> {
    let result = no_conditional_page_query::<Product>(
        &pool,
        "products",
        page_query,
    )
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(err)
        })?;

    log::info!("{} got {} product records of {}/{} page", username, result.data.len(), result.current_page, result.total_pages);

    Ok(Json(result))
}