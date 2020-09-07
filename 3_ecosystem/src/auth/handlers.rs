use actix_identity::Identity;
use actix_web::{
    dev::{Payload, PayloadStream},
    error, web, Error, FromRequest, HttpRequest, HttpResponse, Result,
};
use futures::future::{err, ok, Ready};
use serde::{Deserialize, Serialize};

use crate::auth::password_utils::{hash_password, verify_password};
use crate::db::UserRepo;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserData {
    pub name: String,
    pub password: String,
}

impl FromRequest for UserData {
    type Error = Error;
    type Future = Ready<Result<UserData, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload<PayloadStream>) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, payload).into_inner() {
            if let Some(user_json) = identity.identity() {
                if let Ok(user) = serde_json::from_str(&user_json) {
                    return ok(user);
                }
            }
        }
        err(error::ErrorUnauthorized("Log in first"))
    }
}

pub async fn register_user(
    mut user_data: web::Json<UserData>,
    user_repo: web::Data<UserRepo>,
) -> Result<HttpResponse> {
    let password = std::mem::take(&mut user_data.password);
    let hash = web::block(move || hash_password(password))
        .await
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    user_repo
        .add_user(user_data.name.as_str(), hash.as_str())
        .await
        .map_err(|e| error::ErrorBadRequest(e.msg()))?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn login_user(
    auth_data: web::Json<UserData>,
    id: Identity,
    user_repo: web::Data<UserRepo>,
) -> Result<HttpResponse> {
    let user = user_repo
        .get_user_by_name(auth_data.name.as_str())
        .await
        .map_err(|e| error::ErrorBadRequest(e.msg()))?;

    let (is_correct, auth_data) = web::block(move || {
        verify_password(user.password.as_str(), auth_data.password.as_str())
            .map(|res| (res, auth_data))
    })
    .await
    .map_err(|_| error::ErrorInternalServerError("Try again later"))?;

    if !is_correct {
        return Err(error::ErrorBadRequest("Wrong password"));
    }

    id.remember(serde_json::to_string(&auth_data.into_inner()).unwrap());
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_logged_user(logged_user: UserData) -> HttpResponse {
    HttpResponse::Ok().json(logged_user)
}
