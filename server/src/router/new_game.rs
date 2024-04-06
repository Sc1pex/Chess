use super::*;
use crate::{auth::get_user, components::navbar};
use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use rand::Rng;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Options {
    color: String,
    difficulty: u8,
}

pub async fn get(
    State(state): State<AppState>,
    Query(Options {
        mut color,
        difficulty,
    }): Query<Options>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> impl IntoResponse {
    let user = get_user(&state.pool, TypedHeader(cookies)).await;

    if color == "random" {
        let c: bool = rand::thread_rng().gen();
        if c {
            color = "white".to_string();
        } else {
            color = "black".to_string();
        }
    } else if color != "white" && color != "black" {
        return StatusCode::BAD_REQUEST.into_response();
    }
    if difficulty > 2 {
        return StatusCode::BAD_REQUEST.into_response();
    }

    Html(render_index(html! (
        {navbar(user)}
        <game-el player_color=color difficuly={difficulty}></game-el>
    )))
    .into_response()
}
