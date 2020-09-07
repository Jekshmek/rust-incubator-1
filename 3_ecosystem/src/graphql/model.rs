use juniper::{EmptySubscription, FieldError, FieldResult, RootNode, Value};
use uuid::Uuid;

use crate::db::UserRepo;

impl juniper::Context for UserRepo {}

#[derive(juniper::GraphQLObject)]
struct Friend {
    id: Uuid,
    name: String,
}

struct User {
    id: Uuid,
    name: String,
}

#[juniper::graphql_object(Context = UserRepo)]
impl User {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    async fn friends(&self, context: &UserRepo) -> Vec<Friend> {
        let friends = context
            .get_friends_by_id(&self.id)
            .await
            .expect("Could not get friends");

        friends
            .into_iter()
            .map(|friend| Friend {
                id: friend.id,
                name: friend.name,
            })
            .collect()
    }
}

pub struct Query;

#[juniper::graphql_object(Context = UserRepo)]
impl Query {
    async fn user(name: String, context: &UserRepo) -> FieldResult<User> {
        context
            .get_user_by_name(name.as_str())
            .await
            .map(|user| User {
                id: user.id,
                name: user.name,
            })
            .map_err(|_| FieldError::new("User not found", Value::null()))
    }
}

pub struct Mutations;

#[juniper::graphql_object(Context = UserRepo)]
impl Mutations {
    // async fn add_friend(name: String) -> FieldResult<bool> {
    //     context
    //         .add_friends(user1.as_str(), user2.as_str())
    //         .await
    //         .map(|_| true)
    //         .map_err(|e| {
    //             dbg!(e);
    //             FieldError::new("Error", Value::null())
    //         })
    // }
}

pub type Schema = RootNode<'static, Query, Mutations, EmptySubscription<UserRepo>>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutations, EmptySubscription::new())
}
