use actix_identity::Identity;
use actix_web::{
    dev::{Payload, PayloadStream},
    error, Error, FromRequest, HttpRequest,
};
use futures::future::{err, ok, Ready};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UserLoginData {
    pub name: String,
    pub password: String,
}

impl FromRequest for UserLoginData {
    type Error = Error;
    type Future = Ready<Result<UserLoginData, Error>>;
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
