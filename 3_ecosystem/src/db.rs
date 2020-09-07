use sqlx::{postgres::PgQueryAs, Error, PgPool};

use crate::model::User;
use uuid::Uuid;

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
        sqlx::query_as("SELECT * FROM users WHERE name = $1")
            .bind(name)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_friends_by_id(&self, id: &Uuid) -> Result<Vec<User>, Error> {
        sqlx::query_as(
            "\
            SELECT * FROM user_to_user \
            JOIN users on users.id = user_to_user.user_1 \
            WHERE user_2 = $1\
        ",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn add_friends(&self, name_1: &str, name_2: &str) -> Result<(), Error> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM user WHERE user.id IN ($1, $2)")
            .bind(name_1)
            .bind(name_2)
            .fetch_all(&self.pool)
            .await?;
        
        if users.len() < 2 {
            return Err(Error::Protocol(Box::from("User not found")));
        }

        sqlx::query("\
            INSERT INTO user_to_user VALUES ($1, $2);\
            INSERT INTO user_to_user VALUES ($2, $1)\
        ")
            .bind(&users.get(0).unwrap().id)
            .bind(&users.get(1).unwrap().id)
            .execute(&self.pool)
            .await?;
        
        Ok(())

    }
}
