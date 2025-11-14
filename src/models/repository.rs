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
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RepositoryQueryId {
    pub id: u32,
}

#[derive(Debug, Deserialize)]
pub struct InsertRepository {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRepository {
    pub id: u32,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RepositoryNameQuery {
    pub name: String,
}