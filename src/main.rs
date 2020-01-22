//! Example actix-web application.
//!
//! This code is adapted from the front page of the [Actix][] website.
//!
//! [actix]: https://actix.rs/docs/

use actix_web::http::{header, Method, StatusCode};
use actix_web::middleware::session::{self, RequestSession};
use actix_web::{
    error, fs, middleware, pred, server, App, Error, HttpRequest, HttpResponse, Path, Responder,
    Result,
};
use std::env;
use std::path;

fn welcome(req: &HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}

fn static_data(req: &HttpRequest) -> Result<HttpResponse> {
    let wanted_res = format!("../static/{}", req.match_info().get("name").unwrap_or("42"));
    if path::Path::new(wanted_res).exists() {
        Ok(HttpResponse::build(StatusCode::OK)
            .content_type(req.content_type())
            .body(include_str!(wanted_res)))
    } else {
        Err(HttpResponse::build(StatusCode::NOT_FOUND))
    }
}

fn greet(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

fn main() {
    // Get the port number to listen on.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    // Start a server, configuring the resources to serve.
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(welcome))
            .resource("/static/{res}", |r| r.f(static_data))
            .resource("/{name}", |r| r.f(greet))
    })
    .bind(("0.0.0.0", port))
    .expect("Can not bind to port 8000")
    .run();
}
