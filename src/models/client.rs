use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::models::page::PageQuery;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ClientType {
    Unknown,
    Abnormal,
    Normal,
    Important,
}

#[derive(Debug, Deserialize)]
pub struct ClientTypeQuery {
    pub ctype: ClientType,
}

impl From<String> for ClientType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "abnormal" => ClientType::Abnormal,
            "normal" => ClientType::Normal,
            "important" => ClientType::Important,
            _ => ClientType::Unknown,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
/// 客户信息
pub struct Client {
    /// 客户id
    pub id: u32,
    /// 客户（甲方）名称
    pub name: String,
    /// 客户类型
    pub ctype: ClientType,
    /// 联系人姓名
    pub contactor: String,
    /// 联系人电话
    pub contactor_tel: String,
    /// 通信email
    pub email: Option<String>,
    /// 客户备注
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ClientQueryId {
    pub id: u32,
}

#[derive(Debug, Deserialize)]
pub struct ClientPageQueryId {
    pub id: u32,
    #[serde(flatten)]
    pub page: PageQuery,
}

#[derive(Debug, Deserialize)]
pub struct InsertClient {
    pub name: String,
    pub ctype: ClientType,
    pub contactor: String,
    pub contactor_tel: String,
    pub email: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateClient {
    pub id: u32,
    pub name: Option<String>,
    pub ctype: Option<ClientType>,
    pub contactor: Option<String>,
    pub contactor_tel: Option<String>,
    pub email: Option<String>,
    pub description: Option<String>,
}