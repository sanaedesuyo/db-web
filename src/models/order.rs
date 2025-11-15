use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, sqlx::Type, Debug, )]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum OrderStatus {
    Unpaid,
    Paid,
    Finished,
    Unknown,
}

impl From<String> for OrderStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "unpaid" => OrderStatus::Unpaid,
            "paid" => OrderStatus::Paid,
            "finished" => OrderStatus::Finished,
            _ => OrderStatus::Unknown,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Order {
    /// 订单id
    pub id: u32,
    /// 订单编号
    pub order_id: String,
    /// 订购客户id
    pub cid: u32,
    /// 下单时间
    pub order_time: NaiveDateTime,
    /// 订单状态
    pub status: OrderStatus,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrderItem {
    /// 明细id
    pub id: u32,
    /// 所属订单id
    pub order_id: u32,
    /// 订购产品id
    pub pid: u32,
    /// 订购数量
    pub amount: u32,
    /// 下单时单价
    pub unit_price: u32,
}

#[derive(Debug, Deserialize)]
pub struct InsertOrderItem {
    pub pid: u32,
    pub amount: u32,
    pub unit_price: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderDTO {
    pub order: Order,
    pub order_items: Vec<OrderItemDTO>,
    pub total: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItemDTO {
    pub pid: u32,
    pub amount: u32,
    pub unit_price: u32,
}

impl From<OrderItem> for OrderItemDTO {
    fn from(value: OrderItem) -> Self {
        OrderItemDTO {
            pid: value.pid,
            amount: value.amount,
            unit_price: value.unit_price,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OrderQueryId {
    pub id: u32,
}

#[derive(Debug, Deserialize)]
pub struct InsertOrder {
    pub cid: u32,
    pub order_items: Vec<InsertOrderItem>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrder {
    pub id: u32,
    pub status: OrderStatus,   
}