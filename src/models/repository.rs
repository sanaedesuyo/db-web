use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
/// 仓库信息
pub struct Repository {
    /// 仓库id
    pub id: u32,
    /// 仓库名称
    pub name: String,
    /// 仓库说明
    pub description: String,
}