use axum::{Router, routing::{delete, get, post}};
use sqlx::MySqlPool;

use crate::handlers::{
    client::*,
    inventory::*,
    order::*,
    product::*,
    repository::*,
    user::*,
    cop::user_client::*
};


pub fn client_routes() -> Router<MySqlPool> {
    Router::new()
        .route("/get", get(get_client))
        .route("/add", post(insert_client))
        .route("/update", post(update_client))
        .route("/login", post(client_login))
}

pub fn inventory_routes() -> Router<MySqlPool> {
    Router::new()
        .route("/of_product", get(get_inventory_of_product))
        .route("/of_repo", get(get_inventory_of_repository))
        .route("/add", post(add_inventory))
        .route("/reduce", post(reduce_inventory))
}

pub fn order_routes() -> Router<MySqlPool> {
    Router::new()
        .route("/", get(get_order))
        .route("/page", get(get_orders_page_of_client))
        .route("/add", post(add_order))
        .route("/update", post(update_order))
}

pub fn product_routes() -> Router<MySqlPool> {
    Router::new()
        .route("/get", get(get_product))
        .route("/add", post(insert_product))
        .route("/update", post(update_product))
        .route("/get_all", get(get_all_product))
        .route("/get_page", get(get_product_page))
}

pub fn repository_routes() -> Router<MySqlPool> {
    Router::new()
        .route("/get", get(get_repository))
        .route("/add", post(insert_repository))
        .route("/update", post(update_repository))
        .route("/get_all", get(get_all_repositories))
        .route("/get_by_name_likes", get(get_repository_by_name_likes))
        .route("/delete", delete(delete_repository))
}

pub fn user_routes() -> Router<MySqlPool> {
    Router::new()
        .nest("/cop", user_client_routes())
        .route("/login", post(login))
        .route("/get", get(get_user))
        .route("/delete", delete(delete_user))
        .route("/add", post(insert_user))
        .route("/update", post(update_user))
        .route("/get_all", get(get_page_users))
}

fn user_client_routes() -> Router<MySqlPool> {
    Router::new()
        .route("/", get(user_get_client).post(modify_client_type))
        .route("/all", get(get_all_clients))
        .route("/likes", get(get_clients_by_name_likes))
        .route("/specified", get(get_specified_clients))
}