use super::*;
use crate::{
    auth::get_user,
    components::{games_list, navbar},
    models::Game,
};
use axum::extract::State;

pub async fn get(
    State(state): State<AppState>,
    cookies: TypedHeader<headers::Cookie>,
) -> impl IntoResponse {
    let user = get_user(&state.pool, cookies).await;

    let new_game_text = if user.is_some() {
        "Start new game"
    } else {
        "Play as guest"
    };

    Html(render_index(html! (
        {navbar(user)}
        <div class="content">
            <a class="newgame" href="/new-game">{new_game_text}</a>

            <hr style="width: 80%; margin: 40px 0px;" />
            <h2>"Last 10 games"</h2>
            {last_games(&state).await}
            <a href="/games" style="margin-top: 20px;" class="login">"All games"</a>
        </div>
    )))
}

async fn last_games(state: &AppState) -> String {
    let games = sqlx::query_as!(
        Game,
        r#"
        SELECT games.id, games.played_at, games.result, users.username as "player?"
        FROM games
        LEFT JOIN users ON games.player = users.id
        ORDER BY games.played_at DESC
        LIMIT 10
        "#
    )
    .fetch_all(&state.pool)
    .await
    .unwrap()
    .into_iter()
    .map(|g| g.into())
    .collect();

    games_list(games, None)
}
