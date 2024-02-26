use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct Game {
    pub id: Uuid,
    pub played_at: time::OffsetDateTime,
    pub player: Option<String>,
    pub result: String,
}
