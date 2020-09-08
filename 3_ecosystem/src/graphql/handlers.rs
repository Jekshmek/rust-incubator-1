use actix_identity::Identity;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use juniper_actix::{graphiql_handler, graphql_handler};

use crate::auth::handlers::UserLoginData;
use crate::db::UserRepo;
use crate::graphql::model::{GraphQLContext, Schema};

pub async fn graphql(
    req: HttpRequest,
    payload: web::Payload,
    schema: web::Data<Schema>,
    user_repo: web::Data<UserRepo>,
    login_data: Option<UserLoginData>,
    identity: Identity,
) -> Result<HttpResponse, Error> {
    let authorized_before = login_data.is_some();

    let context = GraphQLContext::new(user_repo.get_ref().clone(), login_data.clone());
    let resp = graphql_handler(&schema, &context, req, payload).await;

    let guard = context.login_data.lock().unwrap();
    let authorized_after = guard.is_some();

    // Check if user was authorized while handling GraphQL
    if !authorized_before && authorized_after {
        identity.remember(serde_json::to_string(&*guard).unwrap());
    }

    // Check if user was re-authorized while handling GraphQL
    if authorized_before && authorized_after {
        if login_data.unwrap() != *guard.as_ref().unwrap() {
            identity.remember(serde_json::to_string(&*guard).unwrap());
        }
    }

    resp
}

pub async fn graphiql() -> Result<HttpResponse, Error> {
    graphiql_handler("/api", None).await
}
