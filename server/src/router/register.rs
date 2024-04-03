use super::*;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
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
            {register_form(None, None, None)}
        </div>
    )))
}

#[derive(serde::Deserialize)]
pub struct RegisterForm {
    username: String,
    password: String,
}

pub async fn post(
    State(state): State<AppState>,
    Form(mut data): Form<RegisterForm>,
) -> impl IntoResponse {
    if data.username.len() < 3 {
        return Html(render_index(html!(
            <div class="content">
                {register_form(None, Some("Username must be at least 3 characters long"), None)}
            </div>
        )))
        .into_response();
    }
    if data.password.len() < 4 {
        return Html(render_index(html!(
            <div class="content">
                {register_form(None, None, Some("Password must be at least 4 characters long"))}
            </div>
        )))
        .into_response();
    }
    data.password = {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(data.password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string()
    };

    let q = sqlx::query!(
        r#"
        INSERT INTO users (username, password)
        VALUES (?, ?)
        "#,
        data.username,
        data.password,
    )
    .execute(&state.pool)
    .await;
    let id = sqlx::query!(
        r#"
        SELECT id FROM users
        WHERE username = ?
        "#,
        data.username,
    )
    .fetch_one(&state.pool)
    .await
    .unwrap()
    .id;

    match q {
        Ok(_) => {
            let cookie = Cookie::build(("SESSION", id.to_string()))
                .path("/")
                .max_age(Duration::days(30))
                .build();
            let mut headers = HeaderMap::new();
            headers.insert(SET_COOKIE, cookie.to_string().parse().unwrap());
            (headers, Redirect::to("/")).into_response()
        }

        Err(e) => {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.kind() == sqlx::error::ErrorKind::UniqueViolation {
                    return Html(render_index(html!(
                            <div class="content">
                                {register_form(None,Some("Username already exists"), None)}
                            </div>
                    )))
                    .into_response();
                }
            }

            error!("Failed to register user: {:?}", e);
            Html(render_index(html!(
                <div class="content">
                    {register_form(None, Some("Failed to register user"), None)}
                </div>
            )))
            .into_response()
        }
    }
}

fn register_form(
    username_value: Option<&str>,
    username_error: Option<&str>,
    password_error: Option<&str>,
) -> String {
    html! {
    <form action="/register" class="register-form" method="post">
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
        <button type="submit" class="register-btn">"Register"</button>
    </form>

    <div class="below-form">
        <a href="/login" class="login">"Already have an account? Login here"</a>
    </div>
    }
}
