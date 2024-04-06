use super::*;
use crate::{
    auth::get_user_id,
    components::game_html,
    models::{Game, User},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use axum_extra::headers::Cookie;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/submit_game", post(submit_game))
        .route("/all_games", get(all_games))
        .route("/user_games", get(user_games))
        .route("/game_moves/:id", get(game_moves))
}

async fn game_moves(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let moves = sqlx::query_scalar!("SELECT moves FROM games WHERE id = ?", id)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(moves))
}

#[derive(serde::Deserialize, Debug)]
struct UserGamesQuery {
    page: i64,
    id: i32,
}

async fn user_games(
    State(state): State<AppState>,
    Query(UserGamesQuery { page, id }): Query<UserGamesQuery>,
) -> Result<Html<String>, StatusCode> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if page < 1 {
        return Err(StatusCode::BAD_REQUEST);
    }
    let games = sqlx::query!(
        r#"
        SELECT games.id, games.played_at, games.result FROM games 
        WHERE player = ? 
        ORDER BY games.played_at DESC
        LIMIT 10
        OFFSET ?"#,
        user.id,
        (page - 1) * 10
    )
    .fetch_all(&state.pool)
    .await
    .unwrap()
    .into_iter()
    .map(|row| crate::models::Game {
        id: row.id,
        played_at: row.played_at,
        player: Some(user.username.clone()),
        result: row.result,
    })
    .collect::<Vec<_>>();

    let next_page = if games.len() < 10 {
        "".to_string()
    } else {
        html! {
            <tr hx-get={format!("/api/user_games?page={}&id={}", page+1, id)} hx-trigger="revealed" hx-swap="outerHTML">
                <td>"Loading"</td>
            </tr>
        }
    };

    Ok(Html(html!(
        { games.into_iter().map(game_html).collect::<String>() }
        { next_page }
    )))
}

#[derive(serde::Deserialize, Debug)]
struct AllGamesQuery {
    page: i64,
}

async fn all_games(
    State(state): State<AppState>,
    Query(AllGamesQuery { page }): Query<AllGamesQuery>,
) -> impl IntoResponse {
    if page < 1 {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let games = sqlx::query_as!(
        Game,
        r#"
        SELECT games.id, games.played_at, games.result, users.username as "player?"
        FROM games
        LEFT JOIN users ON games.player = users.id
        ORDER BY games.played_at DESC
        LIMIT 10
        OFFSET ?"#,
        (page - 1) * 10
    )
    .fetch_all(&state.pool)
    .await
    .unwrap()
    .into_iter()
    .map(|g| g.into())
    .collect::<Vec<Game>>();

    let next_page = if games.len() < 10 {
        "".to_string()
    } else {
        html! {
            <tr hx-get={format!("/api/all_games?page={}", page+1)} hx-trigger="revealed" hx-swap="outerHTML">
                <td>"Loading"</td>
            </tr>
        }
    };

    Html(html!(
        { games.into_iter().map(game_html).collect::<String>() }
        { next_page }
    ))
    .into_response()
}

async fn submit_game(
    State(state): State<AppState>,
    cookies: TypedHeader<Cookie>,
    Json(data): Json<GameDataJson>,
) {
    let user = get_user_id(cookies);
    tracing::error!("Submitting game: {:?}", data);

    match user {
        Some(id) => {
            sqlx::query!(
                "INSERT INTO games (player, moves, result) VALUES (?, ?, ?)",
                id,
                data.moves,
                data.result,
            )
            .execute(&state.pool)
            .await
            .unwrap();
        }
        None => {
            sqlx::query!(
                "INSERT INTO games (moves, result) VALUES (?, ?)",
                data.moves,
                data.result,
            )
            .execute(&state.pool)
            .await
            .unwrap();
        }
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct GameDataJson {
    moves: serde_json::Value,
    result: String,
}
