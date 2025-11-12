use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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