use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow};

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserFlag {
    Unknown,
    Operator,
    Admin,
}

impl From<String> for UserFlag {
    fn from(s: String) -> Self {
        match s.as_str() {
            "operator" => UserFlag::Operator,
            "admin" => UserFlag::Admin,
            _ => UserFlag::Unknown,
        }
    }
}

/// 系统用户
#[derive(Debug, Serialize, Deserialize, FromRow, Decode)]
pub struct User {
    /// 用户id
    pub id: u32,
    /// 用户密码
    pub password: String,
    /// 用户姓名
    pub name: String,
    /// 用户权限级别
    pub flag: UserFlag,
    /// 用户描述
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserDTO {
    pub id: u32,
    pub name: String,
    pub flag: UserFlag,
    pub description: Option<String>,
}

impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        UserDTO {
            id: user.id,
            name: user.name,
            flag: user.flag,
            description: user.description,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct InsertUser {
    pub name: String,
    pub password: String,
    pub flag: UserFlag,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub id: u32,
    pub name: Option<String>,
    pub password: Option<String>,
    pub flag: Option<UserFlag>,
    pub description: Option<String>,
}