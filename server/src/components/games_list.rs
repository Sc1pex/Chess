use crate::models::Game;
use html_to_string_macro::html;

pub fn games_list(games: Vec<Game>, infinite_scroll_url: Option<&str>) -> String {
    html! {
        <table class="games">
            <tr class="games-header">
                <th style="padding: 5px 0px;">"Player"</th>
                <th>"Date"</th>
                <th>"Result"</th>
                <th></th>
            </tr>
            {games.into_iter().map(game_html).collect::<String>()}
            {
                if let Some(url) = infinite_scroll_url {
                    html! {
                        <tr hx-get={url} hx-trigger="revealed" hx-swap="outerHTML">
                            <td>"Loading"</td>
                        </tr>
                    }
                } else {
                    "".into()
                }
            }
        </table>
    }
}

pub fn game_html(game: Game) -> String {
    let date_format = time::format_description::parse("[month repr:short] [day], [year]").unwrap();
    let date = game.played_at.format(&date_format).unwrap().to_string();

    html! {
        <tr class="game" onclick={format!("location.href='/games/{}';", game.id)}>
            <td>{player(&game)}</td>
            <td class="date">{date}</td>
            <td class="result">{game.result}</td>
            <td>
               <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" width="20px" height="20px"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path fill-rule="evenodd" clip-rule="evenodd" d="M14 5C13.4477 5 13 4.55228 13 4C13 3.44772 13.4477 3 14 3H20C20.5523 3 21 3.44772 21 4V10C21 10.5523 20.5523 11 20 11C19.4477 11 19 10.5523 19 10V6.41421L11.7071 13.7071C11.3166 14.0976 10.6834 14.0976 10.2929 13.7071C9.90237 13.3166 9.90237 12.6834 10.2929 12.2929L17.5858 5H14ZM5 7C4.44772 7 4 7.44772 4 8V19C4 19.5523 4.44772 20 5 20H16C16.5523 20 17 19.5523 17 19V14.4375C17 13.8852 17.4477 13.4375 18 13.4375C18.5523 13.4375 19 13.8852 19 14.4375V19C19 20.6569 17.6569 22 16 22H5C3.34315 22 2 20.6569 2 19V8C2 6.34315 3.34315 5 5 5H9.5625C10.1148 5 10.5625 5.44772 10.5625 6C10.5625 6.55228 10.1148 7 9.5625 7H5Z" fill="#ffffff"></path> </g></svg>
            </td>
        </tr>
    }
}

fn player(game: &Game) -> String {
    match &game.player {
        Some(player) => html! {
            <a class="player" href={format!("/users/{}", player)}>{player}</a>
        },
        None => html! {
            "Guest"
        },
    }
}
