use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ClientType {
    Unknown,
    Abnormal,
    Normal,
    Important,
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
    pub client_type: ClientType,
    /// 联系人姓名
    pub contactor: String,
    /// 联系人电话
    pub contactor_tel: String,
    /// 通信email
    pub email: String,
    /// 客户备注
    pub description: String,
}