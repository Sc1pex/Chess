use axum_extra::{headers, TypedHeader};
use sqlx::{MySql, Pool};

pub fn get_user_id(TypedHeader(cookies): TypedHeader<headers::Cookie>) -> Option<i32> {
    cookies.get("SESSION")?.parse().ok()
}

pub async fn get_user_by_id(pool: &Pool<MySql>, id: i32) -> Option<crate::models::User> {
    sqlx::query_as!(
        crate::models::User,
        r#"SELECT * FROM users WHERE id = ?"#,
        id
    )
    .fetch_one(pool)
    .await
    .ok()
    .map(|u| u.into())
}

pub async fn get_user(
    pool: &Pool<MySql>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> Option<crate::models::User> {
    let id = get_user_id(TypedHeader(cookies))?;
    get_user_by_id(pool, id).await
}
