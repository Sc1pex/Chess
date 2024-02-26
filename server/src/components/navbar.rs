use crate::models::User;
use html_to_string_macro::html;

pub fn navbar(user: Option<User>) -> String {
    html!(
        <div class="navbar-wrapper">
            <div class="navbar">
                <a href="/">"Home"</a>

                <div class="user">
                    {match user {
                        Some(user) => logged_in(user),
                        None => not_logged_in(),
                    }}
                </div>
            </div>
        </div>
    )
}

pub fn logged_in(user: User) -> String {
    html!(
        <a href="/logout" class="login">"Logout"</a>
        <a href={format!("/users/{}", &user.username)} class="profile">{&user.username}</a>
    )
}

pub fn not_logged_in() -> String {
    html!(
        <a href="/login" class="login">"Login"</a>
        <a href="/register" class="register">"Sign up"</a>
    )
}
