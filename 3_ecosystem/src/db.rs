use sqlx::{postgres::PgQueryAs, Error, PgPool};

use crate::model::User;

#[derive(Clone, Debug)]
pub struct UserRepo {
    pool: PgPool,
}

impl UserRepo {
    pub fn new(pool: PgPool) -> Self {
        UserRepo { pool }
    }

    pub async fn add_user(&self, name: &str, password: &str) -> Result<u64, Error> {
        sqlx::query("INSERT INTO users (name, password) VALUES ($1, $2)")
            .bind(name)
            .bind(password)
            .execute(&self.pool)
            .await
    }

    pub async fn get_user_by_name(&self, name: &str) -> Result<User, Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE name = $1")
            .bind(name)
            .fetch_one(&self.pool)
            .await
    }
}
