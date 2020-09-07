use actix_web::{web, Error, HttpRequest, HttpResponse};
use juniper_actix::{graphiql_handler as gqli_handler, graphql_handler};

use crate::db::UserRepo;
use crate::graphql::model::Schema;
use crate::handlers::auth::UserData as UserLoginData;

pub async fn graphql(
    req: HttpRequest,
    payload: web::Payload,
    schema: web::Data<Schema>,
    user_repo: web::Data<UserRepo>,
    _login_data: UserLoginData,
) -> Result<HttpResponse, Error> {
    graphql_handler(&schema, user_repo.get_ref(), req, payload).await
}

pub async fn graphiql() -> Result<HttpResponse, Error> {
    gqli_handler("/api", None).await
}
