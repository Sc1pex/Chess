#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub token: Option<String>,
}

#[derive(Debug)]
pub struct Game {
    pub id: i32,
    pub played_at: time::OffsetDateTime,
    pub player: Option<String>,
    pub result: String,
    pub difficulty: Option<String>,
}
