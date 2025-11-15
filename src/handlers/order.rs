use axum::{Json, Router, extract::{Query, State}, routing::{get, post}};
use sqlx::MySqlPool;

use crate::{errors::AppError, middleware::auth::CurrentUser, models::{client::{Client, ClientPageQueryId}, order::{InsertOrder, Order, OrderDTO, OrderItem, OrderQueryId, UpdateOrder}, page::PageResponse}, utils::generation::generate_order_id};

pub async fn get_order(
    State(pool): State<MySqlPool>,
    CurrentUser { .. }: CurrentUser,
    Query(param): Query<OrderQueryId>,
) -> Result<Json<OrderDTO>, Json<AppError>> {
    let order = sqlx::query_as!(
        Order,
        "SELECT * FROM orders WHERE id = ?",
        param.id,
    )
        .fetch_optional(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("数据库查询失败"))
        })?;

    let existed_order = order.ok_or_else(|| {
        Json(AppError::new("该订单不存在"))
    })?;

    let order_items = sqlx::query_as!(
        OrderItem,
        "SELECT * FROM order_items WHERE order_id = ?",
        existed_order.id
    )
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("数据库查询失败"))
        })?;

    let total = order_items
        .iter()
        .map(|item| item.amount * item.unit_price)
        .sum::<u32>();

    let order_items_dto = order_items
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(Json(OrderDTO {
        order: existed_order,
        order_items: order_items_dto,
        total,
    }))
}

pub async fn get_orders_page_of_client(
    State(pool): State<MySqlPool>,
    CurrentUser { .. }: CurrentUser,
    Query(param): Query<ClientPageQueryId>,
) -> Result<Json<PageResponse<OrderDTO>>, Json<AppError>> {
    let offset = (param.page.page - 1) * param.page.page_size;

    let client = sqlx::query_as!(
        Client,
        "SELECT * FROM clients WHERE id = ?",
        param.id
    )
        .fetch_optional(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("数据库查询失败"))
        })?;

    let existed_client = client.ok_or_else(|| {
        Json(AppError::new("该客户不存在"))
    })?;

    let total = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM orders WHERE cid = ?",
        existed_client.id
    )
        .fetch_one(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("数据库查询失败"))
        })?;

    let total_pages = (
        (total as f64) / (param.page.page_size as f64)
    ).ceil() as u64;

    let orders = sqlx::query_as!(
        Order,
        "SELECT * FROM orders WHERE cid = ? LIMIT ? OFFSET ?",
        existed_client.id, param.page.page, offset
    )
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("数据库查询失败"))
        })?;

    let mut result = Vec::new();

    for order in orders {
        let order_items = sqlx::query_as!(
            OrderItem,
            "SELECT * FROM order_items WHERE order_id = ?",
            order.id
        )
            .fetch_all(&pool)
            .await
            .map_err(|err| {
                log::warn!("{}", err);
                Json(AppError::new("数据库查询失败"))
            })?;

        let total = order_items
            .iter()
            .map(|item| item.amount * item.unit_price)
            .sum::<u32>();

        let order_items_dto = order_items
            .into_iter()
            .map(Into::into)
            .collect();

        result.push(OrderDTO {
            order,
            order_items: order_items_dto,
            total
        });
    }

    Ok(Json(PageResponse {
        data: result,
        total: total as u64,
        current_page: param.page.page,
        page_size: param.page.page_size,
        total_pages,
    }))
}

pub async fn add_order(
    State(pool): State<MySqlPool>,
    CurrentUser { .. }: CurrentUser,
    Json(detailed_order): Json<InsertOrder>,
) -> Result<Json<u64>, Json<AppError>> {
    let mut transaction = pool.begin().await.map_err(|err| {
        log::warn!("Failed to start transaction: {}", err);
        Json(AppError::new("数据更新失败，事务未能成功启动"))
    })?;

    let order_uuid = generate_order_id();

    let order_id = sqlx::query!(
        r#"INSERT INTO orders
        (order_id, cid)
        VALUES (?, ?)"#,
        order_uuid, detailed_order.cid
    )
        .execute(&mut *transaction)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("数据更新失败"))
        })?
        .last_insert_id();

    for order_item in detailed_order.order_items {
        sqlx::query!(
            r#"INSERT INTO order_items
            (order_id, pid, amount, unit_price)
            VALUES (?, ?, ?, ?)"#,
            order_id, order_item.pid, order_item.amount, order_item.unit_price
        )
            .execute(&mut *transaction)
            .await
            .map_err(|err| {
                log::warn!("{}", err);
                Json(AppError::new("数据更新失败"))
            })?;
    }

    transaction.commit().await.map_err(|err| {
        log::warn!("Failed to commit transaction: {}", err);
        Json(AppError::new("数据更新失败，事务未能成功提交"))
    })?;

    Ok(Json(order_id))
}

pub async fn update_order(
    State(pool): State<MySqlPool>,
    CurrentUser { .. }: CurrentUser,
    Json(order): Json<UpdateOrder>,
) -> Result<Json<u64>, Json<AppError>> {
    let result = sqlx::query!(
        r#"UPDATE orders SET
        status = ?
        WHERE id = ?"#,
        order.status, order.id
    )
        .execute(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("数据更新失败"))
        })?;

    Ok(Json(result.rows_affected()))
}

pub fn order_routes() -> Router<MySqlPool> {
    Router::new()
        .route("/", get(get_order))
        .route("/page", get(get_orders_page_of_client))
        .route("/add", post(add_order))
        .route("/update", post(update_order))
}