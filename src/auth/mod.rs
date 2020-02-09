use actix_http::{Payload, Response};
use actix_identity::Identity;

use actix_web::http::StatusCode;
use actix_web::{web, FromRequest, HttpRequest, HttpResponse, Result};

use futures::executor::block_on;
use futures::future::{err, ok, Ready};

use serde::Deserialize;

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

            if let Ok(cookie) = cookie.await {
                if let Some(id) = cookie.identity() {
                    crypto.decode(id)
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some(id) = block_on(cmp) {
            ok(LoggedUser { id })
        } else {
            err(HttpResponse::build(StatusCode::NOT_FOUND).into())
        }
    }
}

#[derive(Deserialize)]
pub struct AuthRequest {
    client_id: String,
}

pub async fn login(
    id: Identity,
    data: web::Data<security::Crypto>,
    auth_req: web::Query<AuthRequest>,
) -> HttpResponse {
    let client_id = auth_req.client_id.clone();
    let encoded_token = data.encode(client_id);
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
