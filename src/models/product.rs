use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
/// 产品信息
pub struct Product {
    /// 产品id
    pub id: u32,
    /// 产品名称
    pub name: String,
    /// 产品尺寸
    pub size: String,
    /// 产品参考单价（单位：分）
    pub price: u32,
    /// 产品库存上限（包含）
    pub max_amount: u32,
    /// 产品库存下限（包含）
    pub min_amount: u32,
}

#[derive(Debug, Deserialize)]
pub struct ProductQueryId {
    pub id: u32,
}

#[derive(Debug, Deserialize)]
pub struct InsertProduct {
    pub name: String,
    pub size: String,
    pub price: u32,
    pub max_amount: u32,
    pub min_amount: u32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProduct {
    pub id: u32,
    pub name: Option<String>,
    pub size: Option<String>,
    pub price: Option<u32>,
    pub max_amount: Option<u32>,
    pub min_amount: Option<u32>,
}