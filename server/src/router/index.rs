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
            <form action="/new-game" class="gameopts">
                <button class="newgame" type="submit">{new_game_text}</button>
                <div class="coloropt">
                    <p style="margin: 0">"Play as:"</p>
                    <div style="display: flex">
                        {color_select("white", "checked")}
                        {color_select("random", "")}
                        {color_select("black", "")}
                    </div>
                </div>
            </form>

            <hr style="width: 80%; margin: 40px 0px;" />
            <h2>"Last 10 games"</h2>
            {last_games(&state).await}
            <a href="/games" style="margin-top: 20px;" class="login">"All games"</a>
        </div>
    )))
}

fn color_select(color: &str, checked: &str) -> String {
    html!(
        <div class="tooltip">
            <input type="radio" name="color" id=color value=color style="appearance: none" {checked} />
            <label for=color>
                <img class="colorselect" src=format!("/assets/select-{}.png", color) />
            </label>
            <span class="tooltiptext">{color}</span>
        </div>
    )
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
