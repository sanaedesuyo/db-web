use sqlx::{FromRow, MySqlPool, mysql::MySqlRow};

use crate::{errors::AppError, models::page::{PageQuery, PageResponse}};

pub async fn no_conditional_page_query<T>(
    pool: &MySqlPool,
    table: &str,
    param: PageQuery
) -> Result<PageResponse<T>, AppError> 
where 
    T: for<'r> FromRow<'r, MySqlRow> + Send + Sync + Unpin
{
    let offset = (param.page - 1) * param.page_size;

    let total: i64 = sqlx::query_scalar(
        format!("SELECT COUNT(*) FROM {}", table).as_str()
    )
        .fetch_one(pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            AppError::new("数据库查询错误".into())
        })?;

    let total_pages = (
        (total as f64) / (param.page_size as f64)
    ).ceil() as u64;

    let result = sqlx::query_as::<_, T>(
        format!("SELECT * FROM {} LIMIT ? OFFSET ?", table).as_str(),
    )
        .bind(param.page_size)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(|err| {
            log::warn!("{}", err);
            AppError::new("数据库查询失败".into())
        })?;

    Ok(PageResponse {
        data: result,
        total: total as u64,
        current_page: param.page,
        page_size: param.page_size,
        total_pages,
    })
}