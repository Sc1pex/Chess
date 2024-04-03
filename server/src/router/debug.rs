use super::*;
use crate::{auth::get_user, components::navbar};
use axum::extract::State;

pub async fn get(
    State(state): State<AppState>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> impl IntoResponse {
    let user = get_user(&state.pool, TypedHeader(cookies)).await;

    Html(render_index(html! (
        {navbar(user)}
        <debug-el></debug-el>
    )))
}
