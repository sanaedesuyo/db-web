use uuid::Uuid;

pub fn generate_order_id() -> String {
    format!("{}{}", Uuid::new_v4(), Uuid::new_v4())
}