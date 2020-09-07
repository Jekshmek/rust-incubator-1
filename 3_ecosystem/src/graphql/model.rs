use juniper::{EmptySubscription, FieldError, FieldResult, RootNode, Value};
use uuid::Uuid;

use crate::auth::handlers::UserData as UserLoginData;
use crate::db::UserRepo;

pub struct GraphQLContext {
    pub user_repo: UserRepo,
    pub login_data: Option<UserLoginData>,
}

impl juniper::Context for GraphQLContext {}

#[derive(juniper::GraphQLObject)]
struct Friend {
    id: Uuid,
    name: String,
}

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

    async fn friends(&self, context: &GraphQLContext) -> FieldResult<Vec<Friend>> {
        let friends = context
            .user_repo
            .get_friends_by_id(&self.id)
            .await
            .map_err(|e| FieldError::new(e.to_string(), Value::null()))?;

        Ok(friends
            .into_iter()
            .map(|friend| Friend {
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
        let name_ref = if name.is_empty() {
            context.login_data.as_ref().unwrap().name.as_str()
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
            .map_err(|e| FieldError::new(e.to_string(), Value::null()))
    }
}

pub struct Mutations;

#[juniper::graphql_object(Context = GraphQLContext)]
impl Mutations {
    async fn add_friend(name: String, context: &GraphQLContext) -> FieldResult<bool> {
        context
            .user_repo
            .add_friends(
                context.login_data.as_ref().unwrap().name.as_str(),
                name.as_str(),
            )
            .await
            .map(|_| true)
            .map_err(|e| FieldError::new(e.to_string(), Value::null()))
    }
}

pub type Schema = RootNode<'static, Query, Mutations, EmptySubscription<GraphQLContext>>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutations, EmptySubscription::new())
}
