use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc, Mutex,
};

use actix_web::web;
use juniper::{EmptySubscription, FieldError, FieldResult, RootNode, Value};
use uuid::Uuid;

use crate::auth::password_utils::verify_password;
use crate::auth::{handlers::UserLoginData, password_utils::hash_password};
use crate::db::UserRepo;

pub struct GraphQLContext {
    pub user_repo: UserRepo,
    pub login_data: Arc<Mutex<Option<UserLoginData>>>,
    pub query_depth: AtomicU32,
}

impl GraphQLContext {
    pub fn new(user_repo: UserRepo, login_data: Option<UserLoginData>) -> Self {
        GraphQLContext {
            user_repo,
            login_data: Arc::new(Mutex::new(login_data)),
            query_depth: AtomicU32::new(0),
        }
    }
}

impl juniper::Context for GraphQLContext {}

struct User {
    id: Uuid,
    name: String,
}

#[juniper::graphql_object(Context = GraphQLContext)]
impl User {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    async fn friends(&self, context: &GraphQLContext) -> FieldResult<Vec<User>> {
        if context.query_depth.load(Ordering::Relaxed) > 0 {
            return Err(FieldError::new(
                "Friends depth can not be more than 1",
                Value::null(),
            ));
        }

        context.query_depth.fetch_add(1, Ordering::Relaxed);

        let friends = context
            .user_repo
            .get_friends_by_id(&self.id)
            .await
            .map_err(|e| FieldError::new(e.msg(), Value::null()))?;

        Ok(friends
            .into_iter()
            .map(|friend| User {
                id: friend.id,
                name: friend.name,
            })
            .collect())
    }
}

pub struct Query;

#[juniper::graphql_object(Context = GraphQLContext)]
impl Query {
    #[graphql(arguments(name(default = "".to_owned(),)))]
    async fn user(name: String, context: &GraphQLContext) -> FieldResult<User> {
        let login_data = Option::clone(&context.login_data.lock().unwrap())
            .ok_or_else(|| FieldError::new("Unauthorized", Value::null()))?;

        let name_ref = if name.is_empty() {
            login_data.name.as_str()
        } else {
            name.as_str()
        };

        context
            .user_repo
            .get_user_by_name(name_ref)
            .await
            .map(|user| User {
                id: user.id,
                name: user.name,
            })
            .map_err(|e| FieldError::new(e.msg(), Value::null()))
    }

    async fn login(name: String, password: String, context: &GraphQLContext) -> FieldResult<bool> {
        let user = context
            .user_repo
            .get_user_by_name(name.as_str())
            .await
            .map_err(|e| FieldError::new(e.msg(), Value::null()))?;

        let (correct, password) = web::block(move || {
            verify_password(user.password.as_str(), password.as_str()).map(|b| (b, password))
        })
        .await
        .map_err(|_| FieldError::new("Password error", Value::null()))?;

        if !correct {
            return Err(FieldError::new("Wrong password", Value::null()));
        }

        let mut guard = context.login_data.lock().unwrap();
        guard.replace(UserLoginData { name, password });

        Ok(true)
    }
}

pub struct Mutations;

#[juniper::graphql_object(Context = GraphQLContext)]
impl Mutations {
    async fn add_friend(name: String, context: &GraphQLContext) -> FieldResult<bool> {
        let login_data = Option::clone(&context.login_data.lock().unwrap())
            .ok_or_else(|| FieldError::new("Unauthorized", Value::null()))?;

        context
            .user_repo
            .add_friends(login_data.name.as_str(), name.as_str())
            .await
            .map(|_| true)
            .map_err(|e| FieldError::new(e.msg(), Value::null()))
    }

    async fn add_user(
        name: String,
        password: String,
        context: &GraphQLContext,
    ) -> FieldResult<bool> {
        let hash = web::block(move || hash_password(password))
            .await
            .map_err(|_| FieldError::new("Password error", Value::null()))?;

        context
            .user_repo
            .add_user(name.as_str(), hash.as_str())
            .await
            .map(|_| true)
            .map_err(|e| FieldError::new(e.msg(), Value::null()))
    }
}

pub type Schema = RootNode<'static, Query, Mutations, EmptySubscription<GraphQLContext>>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutations, EmptySubscription::new())
}
