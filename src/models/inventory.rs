use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, mysql::MySqlRow};

use crate::models::product::Product;

#[derive(Debug, Serialize, Deserialize, FromRow)]
/// 库存订单
pub struct Inventory {
    /// 产品id
    pub pid: u32,
    /// 所属仓库id
    pub rid: u32,
    /// 库存数量
    pub amount: u32,
}

#[derive(Debug, Deserialize)]
pub struct InventoryRepoQueryId {
    pub rid: u32,
}

#[derive(Debug, Deserialize)]
pub struct InventoryProductQueryId {
    pub pid: u32,
}

#[derive(Debug, Serialize)]
pub struct InventoryDetail {
    pub rid: u32,
    pub rname: String,
    pub product: Product,
    pub amount: u32,
}

impl<'r> FromRow<'r, MySqlRow> for InventoryDetail {
    fn from_row(row: &'r MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(InventoryDetail {
            rid: row.try_get("rid")?,
            rname: row.try_get("rname")?,
            product: Product {
                id: row.try_get("pid")?,
                name: row.try_get("pname")?,
                size: row.try_get("psize")?,
                price: row.try_get("pprice")?,
                max_amount: row.try_get("pmax_amount")?,
                min_amount: row.try_get("pmin_amount")?,
            },
            amount: row.try_get("amount")?,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct AddInventory {
    pub rid: u32,
    pub pid: u32,
    pub amount: u32,
}

#[derive(Debug, Deserialize)]
pub struct ReduceInventory {
    pub rid: u32,
    pub pid: u32,
    pub amount: u32,
}