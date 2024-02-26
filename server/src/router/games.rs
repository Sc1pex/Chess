use super::*;
use crate::{
    auth::get_user,
    components::{games_list, navbar},
    models::Game,
};
use axum::extract::{Path, State};
use uuid::Uuid;

pub async fn get(
    State(state): State<AppState>,
    cookies: TypedHeader<headers::Cookie>,
) -> impl IntoResponse {
    let games = sqlx::query_as!(
        Game,
        r#"
        SELECT games.id, games.played_at, games.result, users.username as "player?"
        FROM games
        LEFT JOIN users ON games.player = users.id
        ORDER BY games.played_at DESC
        LIMIT 10
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .unwrap();

    Html(render_index(html! (
        {navbar(get_user(&state.pool, cookies).await)}
        <div class="content">
            <h1>"All games played"</h1>
            {games_list(games, Some("/api/all_games?page=2"))}
        </div>
    )))
}

pub async fn get_game(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    cookies: TypedHeader<headers::Cookie>,
) -> impl IntoResponse {
    Html(render_index(html! (
        {navbar(get_user(&state.pool, cookies).await)}
        <game-el game_id={id}></game-el>
    )))
}
