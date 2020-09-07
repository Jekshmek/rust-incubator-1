use actix_web::{web, Error, HttpRequest, HttpResponse};
use juniper_actix::{graphiql_handler, graphql_handler};

use crate::auth::handlers::UserData as UserLoginData;
use crate::db::UserRepo;
use crate::graphql::model::{GraphQLContext, Schema};

pub async fn graphql(
    req: HttpRequest,
    payload: web::Payload,
    schema: web::Data<Schema>,
    user_repo: web::Data<UserRepo>,
    login_data: UserLoginData,
) -> Result<HttpResponse, Error> {
    let context = GraphQLContext::new(user_repo.get_ref().clone(), Some(login_data));

    graphql_handler(&schema, &context, req, payload).await
}

pub async fn graphiql() -> Result<HttpResponse, Error> {
    graphiql_handler("/api", None).await
}
