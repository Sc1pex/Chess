use axum::{
    http::header::SET_COOKIE,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use axum_extra::{headers, TypedHeader};
use html_to_string_macro::html;
use sqlx::{MySql, Pool};
use tower_http::{services::ServeDir, trace::TraceLayer};

mod api;
mod debug;
mod games;
mod index;
mod login;
mod new_game;
mod register;
mod user;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<MySql>,
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/", get(index::get))
        .route("/new-game", get(new_game::get))
        .route("/debug", get(debug::get))
        .route("/register", get(register::get))
        .route("/register", post(register::post))
        .route("/login", get(login::get))
        .route("/login", post(login::post))
        .route("/logout", get(logout))
        .route("/users/:username", get(user::get))
        .route("/games", get(games::get))
        .route("/games/:id", get(games::get_game))
        .nest("/api", api::router())
        .with_state(state)
        .nest_service(
            "/assets",
            ServeDir::new(std::env::var("ASSETS_DIR").unwrap()),
        )
        .layer(TraceLayer::new_for_http())
}

fn render_index(body: String) -> String {
    let index_html = include_str!(env!("INDEX_HTML"));
    let (p1, p2) = index_html
        .split_once("<!-- content -->")
        .expect("index.html is invalid");
    format!("{}{}{}", p1, body, p2)
}

async fn logout() -> impl IntoResponse {
    let cookie = "SESSION=; HttpOnly; Path=/; Max-Age=0";
    let mut headers = headers::HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.parse().unwrap());
    (headers, Redirect::to("/"))
}
