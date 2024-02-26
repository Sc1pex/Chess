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
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/submit_game", post(submit_game))
        .route("/all_games", get(all_games))
        .route("/user_games", get(user_games))
        .route("/game_moves/:id", get(game_moves))
}

async fn game_moves(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let moves = sqlx::query_scalar!("SELECT moves FROM games WHERE id = $1", id)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(moves))
}

#[derive(serde::Deserialize, Debug)]
struct UserGamesQuery {
    page: i64,
    id: Uuid,
}

async fn user_games(
    State(state): State<AppState>,
    Query(UserGamesQuery { page, id }): Query<UserGamesQuery>,
) -> Result<Html<String>, StatusCode> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    if page < 1 {
        return Err(StatusCode::BAD_REQUEST);
    }
    let games = sqlx::query!(
        r#"
        SELECT games.id, games.played_at, games.result FROM games 
        WHERE player = $1 
        ORDER BY games.played_at DESC
        LIMIT 10
        OFFSET $2 ROWS"#,
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
        OFFSET $1 ROWS
        "#,
        (page - 1) * 10
    )
    .fetch_all(&state.pool)
    .await
    .unwrap();

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

    match user {
        Some(id) => {
            sqlx::query!(
                "INSERT INTO games (id, player, moves, result) VALUES ($1, $2, $3, $4)",
                uuid::Uuid::new_v4(),
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
                "INSERT INTO games (id, moves, result) VALUES ($1, $2, $3)",
                uuid::Uuid::new_v4(),
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
