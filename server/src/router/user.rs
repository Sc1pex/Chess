use super::*;
use crate::{
    auth::get_user,
    components::{games_list, navbar},
    models::User,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use std::collections::HashMap;

pub async fn get(
    cookies: TypedHeader<headers::Cookie>,
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = ?", username)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "User not found"))?
        .into();

    let stats = get_stats(&user, &state).await;
    let games = sqlx::query!(
        r#"
        SELECT games.id, games.played_at, games.result FROM games 
        WHERE player = ? 
        ORDER BY games.played_at DESC
        LIMIT 10"#,
        user.id
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

    Ok(Html(render_index(html! (
        {navbar(get_user(&state.pool, cookies).await)}
        <div class="content">
            <h1>{&user.username}</h1>
            {stats_html(stats)}
            <h2>"Games"</h2>
            {games_list(games, Some(&format!("/api/user_games?page=2&id={}", user.id)))}
        </div>
    ))))
}

fn stats_html(stats: [i64; 3]) -> String {
    html! (
        <div class="stats">
            <h2>"Stats"</h2>
            <p>
                "Wins: " {stats[0]}
            </p>
            <p>
                "Draws: " {stats[1]}
            </p>
            <p>
                "Losses: " {stats[2]}
            </p>
        </div>
    )
}

async fn get_stats(user: &User, state: &AppState) -> [i64; 3] {
    let stats = sqlx::query!(
        r#"SELECT result, count(result) as count FROM games WHERE player = ? GROUP BY result ORDER BY result DESC"#,
        user.id
    )
    .fetch_all(&state.pool)
    .await
    .unwrap()
    .into_iter()
    .map(|row| (row.result, row.count))
    .collect::<HashMap<_, _>>();

    [
        stats.get("Win").copied().unwrap_or(0),
        stats.get("Draw").copied().unwrap_or(0),
        stats.get("Loss").copied().unwrap_or(0),
    ]
}
