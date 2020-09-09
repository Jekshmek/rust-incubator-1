use actix_identity::Identity;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};

use crate::auth::model::UserLoginData;
use crate::db::repository::UserRepo;
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
    let login_data_after = &*guard;

    let authorized_after = login_data_after.is_some();

    let was_authorized = !authorized_before && authorized_after;
    let was_reauthorized = authorized_before
        && authorized_after
        && login_data.unwrap() != *login_data_after.as_ref().unwrap();

    if was_authorized || was_reauthorized {
        let json = serde_json::to_string(login_data_after.as_ref().unwrap()).unwrap();
        identity.remember(json);
    }

    let was_logged_out = authorized_before && !authorized_after;
    if was_logged_out {
        identity.forget();
    }

    resp
}

pub async fn graphiql() -> Result<HttpResponse, Error> {
    graphiql_handler("/api", None).await
}

pub async fn playground() -> Result<HttpResponse, Error> {
    playground_handler("/api", None).await
}
