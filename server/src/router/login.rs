use crate::models::User;

use super::*;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::State,
    http::{header::SET_COOKIE, HeaderMap},
    response::Redirect,
    Form,
};
use axum_extra::extract::cookie::Cookie;
use time::Duration;
use tracing::error;

pub async fn get() -> impl IntoResponse {
    Html(render_index(html!(
        <div class="content">
            {login_form(None, None, None)}
        </div>
    )))
}

#[derive(serde::Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub async fn post(State(state): State<AppState>, Form(data): Form<LoginForm>) -> impl IntoResponse {
    if data.username.len() < 3 {
        return Html(render_index(html!(
            <div class="content">
                {login_form(None, Some("Username must be at least 3 characters long"), None)}
            </div>
        )))
        .into_response();
    }
    if data.password.len() < 4 {
        return Html(render_index(html!(
            <div class="content">
                {login_form(None, None, Some("Password must be at least 4 characters long"))}
            </div>
        )))
        .into_response();
    }

    let q = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users
        WHERE username = ? 
        "#,
        data.username
    )
    .fetch_one(&state.pool)
    .await;

    match q {
        Ok(user) => {
            let password = Argon2::default()
                .verify_password(
                    data.password.as_bytes(),
                    &PasswordHash::new(&user.password).unwrap(),
                )
                .is_ok();
            if password {
                let cookie = Cookie::build(("SESSION", user.id.to_string()))
                    .path("/")
                    .max_age(Duration::days(30))
                    .build();
                let mut headers = HeaderMap::new();
                headers.insert(SET_COOKIE, cookie.to_string().parse().unwrap());
                (headers, Redirect::to("/")).into_response()
            } else {
                Html(render_index(html!(
                    <div class="content">
                        {login_form(Some(&data.username), None, Some("Invalid password"))}
                    </div>
                )))
                .into_response()
            }
        }

        Err(e) => {
            if let sqlx::Error::RowNotFound = e {
                return Html(render_index(html!(
                        <div class="content">
                            {login_form(None,Some("Username dosen't exist"), None)}
                        </div>
                )))
                .into_response();
            }

            error!("Failed to register user: {:?}", e);
            Html(render_index(html!(
                <div class="content">
                    {login_form(None, Some("Failed to register user"), None)}
                </div>
            )))
            .into_response()
        }
    }
}

fn login_form(
    username_value: Option<&str>,
    username_error: Option<&str>,
    password_error: Option<&str>,
) -> String {
    html! {
    <form action="/login" class="register-form" method="post">
        <div class="field-div">
            <input
                type="text"
                id="username"
                name="username"
                placeholder="Username: "
                class="field"
                value={username_value.unwrap_or("")}
                required
            />
            <span class="error">{username_error.unwrap_or("")}</span>
        </div>
        <div class="field-div">
            <input
                type="password"
                id="password"
                name="password"
                placeholder="Password: "
                class="field"
                required
            />
            <span class="error">{password_error.unwrap_or("")}</span>
        </div>
        <button type="submit" class="register-btn">"Login"</button>
    </form>

    <div class="below-form">
        <a href="/register" class="login">"Don't have an account? Register here"</a>
    </div>
    }
}
