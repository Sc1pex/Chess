use axum_extra::{headers, TypedHeader};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub fn get_user_id(TypedHeader(cookies): TypedHeader<headers::Cookie>) -> Option<Uuid> {
    cookies.get("SESSION")?.parse().ok()
}

pub async fn get_user_by_id(pool: &Pool<Postgres>, id: Uuid) -> Option<crate::models::User> {
    sqlx::query_as!(
        crate::models::User,
        r#"SELECT * FROM users WHERE id = $1"#,
        id
    )
    .fetch_one(pool)
    .await
    .ok()
}

pub async fn get_user(
    pool: &Pool<Postgres>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> Option<crate::models::User> {
    let id = get_user_id(TypedHeader(cookies))?;
    get_user_by_id(pool, id).await
}
