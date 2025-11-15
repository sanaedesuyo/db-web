use axum::{Json, extract::{Query, State}};
use sqlx::{MySqlPool};

use crate::{errors::AppError, middleware::auth::CurrentUser, models::inventory::{
    AddInventory, Inventory, InventoryDetail, InventoryProductQueryId, InventoryRepoQueryId, ReduceInventory
}};

pub async fn get_inventory_of_repository(
    State(pool): State<MySqlPool>,
    CurrentUser { .. }: CurrentUser,
    Query(param): Query<InventoryRepoQueryId>,
) -> Result<Json<Vec<InventoryDetail>>, Json<AppError>> {
    let result = sqlx::query_as::<_, InventoryDetail>(
        r#"SELECT 
        ti.rid,
        ti.pid,
        tp.name AS pname,
        tp.size AS psize,
        tp.price AS pprice,
        tp.max_amount AS pmax_amount,
        tp.min_amount AS pmin_amount,
        tr.name AS rname,
        amount
        FROM inventory AS ti, products AS tp, repository AS tr
        WHERE rid = ? AND ti.rid = tr.id AND ti.pid = tp.id"#
    )
        .bind(param.rid)
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("无法获取库存信息"))
        })?;

    Ok(Json(result))
}

pub async fn get_inventory_of_product(
    State(pool): State<MySqlPool>,
    CurrentUser { .. }: CurrentUser,
    Query(param): Query<InventoryProductQueryId>,
) -> Result<Json<Vec<InventoryDetail>>, Json<AppError>> {
    let result = sqlx::query_as::<_, InventoryDetail>(
        r#"SELECT 
        ti.rid,
        ti.pid,
        tp.name AS pname,
        tp.size AS psize,
        tp.price AS pprice,
        tp.max_amount AS pmax_amount,
        tp.min_amount AS pmin_amount,
        tr.name AS rname,
        amount
        FROM inventory AS ti, products AS tp, repository AS tr
        WHERE pid = ? AND ti.pid = tp.id AND ti.rid = tr.id"#
    )
        .bind(param.pid)
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("无法获取库存信息"))
        })?;

    Ok(Json(result))
}

pub async fn add_inventory(
    State(pool): State<MySqlPool>,
    CurrentUser { .. }: CurrentUser,
    Json(inventory): Json<AddInventory>,
) -> Result<Json<u64>, Json<AppError>> {
    let mut transaction = pool.begin().await.map_err(|err| {
        log::warn!("Failed to start transaction: {}", err);
        Json(AppError::new("事务启动失败"))
    })?;

    let existing = sqlx::query_as!(
        Inventory,
        "SELECT * FROM inventory WHERE rid = ? AND pid = ?",
        inventory.rid, inventory.pid
    )
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("查询库存信息时失败"))
        })?;
    
    if existing.is_none() {
        sqlx::query!(
            "INSERT INTO inventory (rid, pid, amount) VALUES(?, ?, ?)",
            inventory.rid, inventory.pid, 0u32
        )
            .execute(&mut *transaction)
            .await
            .map_err(|err| {
                log::warn!("{}", err);
                Json(AppError::new("创建新库存信息时失败"))
            })?;
    }

    let result = sqlx::query!(
        r#"UPDATE inventory SET
        amount = amount + ?
        WHERE rid = ? AND pid = ?"#,
        inventory.amount, inventory.rid, inventory.pid
    )
        .execute(&mut *transaction)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("更新库存信息时失败"))
        })?;

    transaction.commit().await.map_err(|err| {
        log::warn!("Failed to commit transaction: {}", err);
        Json(AppError::new("更新失败，事务未能成功提交"))
    })?;

    Ok(Json(result.rows_affected()))
}

pub async fn reduce_inventory(
    State(pool): State<MySqlPool>,
    CurrentUser { .. }: CurrentUser,
    Json(inventory): Json<ReduceInventory>,
) -> Result<Json<u64>, Json<AppError>> {
    let mut transaction = pool.begin().await.map_err(|err| {
        log::warn!("Failed to start transaction: {}", err);
        Json(AppError::new("事务启动失败"))
    })?;

    let record = sqlx::query_as!(
        Inventory,
        "SELECT * FROM inventory WHERE rid = ? AND pid = ?",
        inventory.rid, inventory.pid
    )
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("查询库存时失败"))
        })?;

    match record {
        Some(existed_inventory) => {
            if existed_inventory.amount < inventory.amount {
                return Err(Json(AppError::new("库存数量不足，操作失败")));
            }
        },
        None => {
            return Err(Json(AppError::new("仓库中不存在此产品")))
        }
    }

    let result = sqlx::query!(
        r#"UPDATE inventory SET
        amount = amount - ?
        WHERE rid = ? AND pid = ?"#,
        inventory.amount, inventory.rid, inventory.pid
    )
        .execute(&mut *transaction)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            Json(AppError::new("更新库存时失败"))
        })?;

    transaction.commit().await.map_err(|err| {
        log::warn!("Failed to commit transaction: {}", err);
        Json(AppError::new("更新失败，事务未能成功提交"))
    })?;
    
    Ok(Json(result.rows_affected()))
}
