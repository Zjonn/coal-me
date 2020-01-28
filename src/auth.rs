use actix_http::{Payload, Response};
use actix_identity::Identity;

use actix_web::http::StatusCode;
use actix_web::{FromRequest, HttpRequest, HttpResponse, Result};

use futures::executor::block_on;
use futures::future::{err, ok, Ready};

pub struct LoggedUser {
    id: String,
}

impl FromRequest for LoggedUser {
    type Error = HttpResponse;
    type Config = ();
    type Future = Ready<Result<Self, HttpResponse>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        if let Some(id) = block_on(Identity::from_request(req, payload))
            .unwrap()
            .identity()
        {
            ok(LoggedUser { id })
        } else {
            err(HttpResponse::build(StatusCode::NOT_FOUND).into())
        }
    }
}

pub async fn login(id: Identity) -> HttpResponse {
    id.remember("XD".parse().unwrap());
    Response::build(StatusCode::OK).finish()
}

pub async fn logged(_: LoggedUser) -> HttpResponse {
    HttpResponse::build(StatusCode::OK).finish()
}

pub async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    Response::build(StatusCode::OK).finish()
}
