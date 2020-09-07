use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password: String,
}

#[derive(sqlx::FromRow)]
pub struct UserToUser {
    pub user_1: Uuid,
    pub user_2: Uuid,
}
