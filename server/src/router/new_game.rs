use super::*;
use crate::{auth::get_user, components::navbar};
use axum::extract::{Query, State};
use rand::Rng;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Options {
    color: String,
}

pub async fn get(
    State(state): State<AppState>,
    Query(Options { mut color }): Query<Options>,
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
    }
    tracing::info!("New game with color: {}", color);

    Html(render_index(html! (
        {navbar(user)}
        <game-el player_color=color></game-el>
    )))
}
