use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, mysql::MySqlRow};

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

impl From<ClientType> for String {
    fn from(value: ClientType) -> Self {
        match value {
            ClientType::Abnormal => "abnormal",
            ClientType::Normal => "normal",
            ClientType::Important => "important",
            _ => "unknown",
        }.into()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// 客户信息
pub struct Client {
    /// 客户id
    pub id: u32,
    /// 客户（甲方）名称
    pub name: String,
    /// 客户账号名
    pub username: String, 
    /// 客户账号密码
    pub password: String,
    /// 客户类型
    pub ctype: ClientType,
    /// 联系人姓名
    pub contactor: String,
    /// 联系人电话
    pub contactor_tel: String,
    /// 通信email
    pub email: String,
    /// 客户备注
    pub description: Option<String>,
}

impl<'r> FromRow<'r, MySqlRow> for Client {
    fn from_row(row: &'r MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Client {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            username: row.try_get("username")?,
            password: row.try_get("password")?,
            ctype: ClientType::from(row.try_get::<String, _>("ctype")?),
            contactor: row.try_get("contactor")?,
            contactor_tel: row.try_get("contactor_tel")?,
            email: row.try_get("email")?,
            description: row.try_get("description")?,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct ClientQueryId {
    pub id: u32,
}

#[derive(Debug, Deserialize)]
pub struct ClientPageQueryId {
    pub id: u32,
    pub page_size: u64,
    pub page: u64,
}

#[derive(Debug, Deserialize)]
pub struct InsertClient {
    pub name: String,
    pub username: String,
    pub password: String,
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
    pub password: Option<String>,
    pub ctype: Option<ClientType>,
    pub contactor: Option<String>,
    pub contactor_tel: Option<String>,
    pub email: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginClient {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct ClientDTO {
    pub name: String,
    pub username: String, 
    pub ctype: ClientType,
    pub contactor: String,
    pub contactor_tel: String,
    pub email: String,
    pub description: Option<String>,
}

impl From<Client> for ClientDTO {
    fn from(c: Client) -> Self {
        ClientDTO {
            name: c.name,
            username: c.username,
            ctype: c.ctype,
            contactor: c.contactor,
            contactor_tel: c.contactor_tel,
            email: c.email,
            description: c.description,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ClientTypeModifyQuery {
    pub id: u32,
    pub ctype: ClientType,
}