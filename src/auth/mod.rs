use actix_http::{Payload, Response};
use actix_identity::Identity;

use actix_web::http::StatusCode;
use actix_web::{web, FromRequest, HttpRequest, HttpResponse, Result};

use futures::executor::block_on;
use futures::future::{err, ok, Ready};

pub mod security;

pub struct LoggedUser {
    id: String,
}

impl FromRequest for LoggedUser {
    type Error = HttpResponse;
    type Config = ();
    type Future = Ready<Result<Self, HttpResponse>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let crypto = web::Data::<security::Crypto>::from_request(req, payload);
        let cookie = Identity::from_request(req, payload);

        let cmp = async {
            let crypto = crypto.await.expect("Server corrupted!");

            if let Some(id) = cookie.await.unwrap().identity() {
                let decoded_id = crypto.decode(id);
                Some(decoded_id)
            } else {
                None
            }
        };

        if let Some(id) = block_on(cmp).unwrap() {
            ok(LoggedUser { id })
        } else {
            err(HttpResponse::build(StatusCode::NOT_FOUND).into())
        }
    }
}

pub trait EncodeLoggedUser: Sized {
    fn encode(&self, data: String) -> String;
    fn decode(&self, data: String) -> Option<String>;
}

pub async fn login(id: Identity, data: web::Data<security::Crypto>) -> HttpResponse {
    let encoded_token = data.encode("XD".parse().unwrap());
    id.remember(encoded_token);
    Response::build(StatusCode::OK).finish()
}

pub async fn logged(_: LoggedUser) -> HttpResponse {
    HttpResponse::build(StatusCode::OK).finish()
}

pub async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    Response::build(StatusCode::OK).finish()
}
