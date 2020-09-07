use sqlx::{postgres::PgQueryAs, PgPool};
use uuid::Uuid;

use crate::model::User;

#[derive(Clone, Debug)]
pub struct UserRepo {
    pool: PgPool,
}

impl UserRepo {
    pub fn new(pool: PgPool) -> Self {
        UserRepo { pool }
    }

    pub async fn add_user(&self, name: &str, password: &str) -> Result<u64, UserRepoError> {
        sqlx::query("INSERT INTO users (name, password) VALUES ($1, $2)")
            .bind(name)
            .bind(password)
            .execute(&self.pool)
            .await
            .map_err(|_| UserRepoError::UserAlreadyExists)
    }

    pub async fn get_user_by_name(&self, name: &str) -> Result<User, UserRepoError> {
        sqlx::query_as("SELECT * FROM users WHERE name = $1")
            .bind(name)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| UserRepoError::UserNotFound)
    }

    pub async fn get_friends_by_id(&self, id: &Uuid) -> Result<Vec<User>, UserRepoError> {
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
        .map_err(|_| UserRepoError::UserNotFound)
    }

    pub async fn add_friends(&self, name_1: &str, name_2: &str) -> Result<(), UserRepoError> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users WHERE users.name IN ($1, $2)")
            .bind(name_1)
            .bind(name_2)
            .fetch_all(&self.pool)
            .await
            .map_err(|_| UserRepoError::UserNotFound)?;

        if users.len() < 2 {
            return Err(UserRepoError::UserNotFound);
        }

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|_| UserRepoError::ConnectionFailed)?;

        sqlx::query("INSERT INTO user_to_user VALUES ($1, $2)")
            .bind(&users.get(0).unwrap().id)
            .bind(&users.get(1).unwrap().id)
            .execute(&mut tx)
            .await
            .map_err(|_| UserRepoError::AlreadyFriends)?;

        sqlx::query("INSERT INTO user_to_user VALUES ($1, $2)")
            .bind(&users.get(1).unwrap().id)
            .bind(&users.get(0).unwrap().id)
            .execute(&mut tx)
            .await
            .map_err(|_| UserRepoError::AlreadyFriends)?;

        tx.commit()
            .await
            .map_err(|_| UserRepoError::AlreadyFriends)?;

        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum UserRepoError {
    UserNotFound,
    UserAlreadyExists,
    AlreadyFriends,
    ConnectionFailed,
}

impl ToString for UserRepoError {
    fn to_string(&self) -> String {
        match self {
            UserRepoError::UserNotFound => "User not found".to_string(),
            UserRepoError::UserAlreadyExists => "User already exists".to_string(),
            UserRepoError::AlreadyFriends => "Users are already friends".to_string(),
            UserRepoError::ConnectionFailed => "Failed to establish connection".to_string(),
        }
    }
}
