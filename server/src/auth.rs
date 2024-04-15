use axum_extra::{headers, TypedHeader};
use sqlx::{MySql, Pool};

pub fn get_user_token(TypedHeader(cookies): TypedHeader<headers::Cookie>) -> Option<String> {
    cookies.get("SESSION")?.parse().ok()
}

pub async fn get_user_by_token(pool: &Pool<MySql>, token: String) -> Option<crate::models::User> {
    sqlx::query_as!(
        crate::models::User,
        r#"SELECT * FROM users WHERE token = ?"#,
        token
    )
    .fetch_one(pool)
    .await
    .ok()
}

pub async fn get_user(
    pool: &Pool<MySql>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> Option<crate::models::User> {
    let token = get_user_token(TypedHeader(cookies))?;
    get_user_by_token(pool, token).await
}

pub async fn update_token(pool: &Pool<MySql>, id: i32) -> Option<String> {
    let token = uuid::Uuid::new_v4().to_string();
    sqlx::query!(r#"UPDATE users SET token = ? where id = ?"#, token, id)
        .execute(pool)
        .await
        .ok()?;

    Some(token)
}
