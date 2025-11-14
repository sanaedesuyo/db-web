use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}

fn default_page_size() -> u64 {
    10
}

#[derive(Debug, Serialize)]
pub struct PageResponse<T> {
    /// 数据列表
    pub data: Vec<T>,
    /// 总条数
    pub total: u64,
    /// 当前页码
    pub current_page: u64,
    /// 每页数量
    pub page_size: u64,
    /// 总页数
    pub total_pages: u64
}