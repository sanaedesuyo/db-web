use axum::{Json, Router, extract::{Query, State}, routing::{get, post}};
use sqlx::MySqlPool;
use crate::{errors::AppError, models::{page::{PageQuery, PageResponse}, product::*}};

pub async fn get_product(
    State(pool): State<MySqlPool>,
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
            Json(AppError::new("找不到该产品".into()))
        })?;

    Ok(Json(result))
}

pub async fn insert_product(
    State(pool): State<MySqlPool>,
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
            Json(AppError::new("添加产品失败".into()))
        })?;

    Ok(Json(result.last_insert_id()))
}

pub async fn update_product(
    State(pool): State<MySqlPool>,
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
            Json(AppError::new("更新产品信息失败".into()))
        })?;

    Ok(Json(result.rows_affected()))
}

pub async fn get_all_product(
    State(pool): State<MySqlPool>
) -> Result<Json<Vec<Product>>, Json<AppError>> {
    let result = sqlx::query_as!(
        Product,
        "SELECT * FROM products",
    )
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("无法获取产品信息列表".into()))
        })?;

    Ok(Json(result))
}

pub async fn get_product_page(
    State(pool): State<MySqlPool>,
    Query(page_query): Query<PageQuery>,
) -> Result<Json<PageResponse<Product>>, Json<AppError>> {
    let offset = (page_query.page - 1) * page_query.page_size;

    let total = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM products"
    )
        .fetch_one(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("获取产品总数失败".into()))
        })?;
    
    let products = sqlx::query_as!(
        Product,
        "SELECT * FROM products LIMIT ? OFFSET ?",
        page_query.page_size, offset
    )
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("获取产品列表失败".into()))
        })?;

    let total_pages = (
        (total as f64) / (page_query.page_size as f64)
    ).ceil() as u64;

    Ok(Json(PageResponse {
        data: products,
        total: total as u64,
        current_page: page_query.page,
        page_size: page_query.page_size,
        total_pages,
    }))
}

pub fn product_routes() -> Router<MySqlPool> {
    Router::new()
        .route("/get", get(get_product))
        .route("/add", post(insert_product))
        .route("/update", post(update_product))
        .route("/get_all", get(get_all_product))
        .route("/get_page", get(get_product_page))
}