//! Example actix-web application.
//!
//! This code is adapted from the front page of the [Actix][] website.
//!
//! [actix]: https://actix.rs/docs/

#[macro_use]
extern crate lazy_static;

use chrono;

use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};

use actix_web::http::{StatusCode};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};

use std::env;

// fn welcome(req: &HttpRequest) -> Result<HttpResponse> {
//     Ok(HttpResponse::build(StatusCode::OK)
//         .content_type("text/html; charset=utf-8")
//         .body(include_str!("../static/index.html")))
// }

// fn static_data(req: &HttpRequest) -> Result<HttpResponse> {
//     Ok(HttpResponse::build(StatusCode::OK)
//         .content_type("text/html; charset=utf-8")
//         .body(include_str!("../static/index.html")))
// }

// fn greet(req: &HttpRequest) -> impl Responder {
//     let to = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", to)
// }

lazy_static::lazy_static! {
pub  static ref SECRET_KEY: String = "CoalME".repeat(8);
}

fn main() {
    // Get the port number to listen on.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    // Start a server, configuring the resources to serve.
    HttpServer::new(|| {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .secure(true)
                    .domain("https://coal-me.herokuapp.com/")
                    .max_age_time(chrono::Duration::hours(12)),
            ))
            .service(fs::Files::new("/logged", "./static/logged").index_file("index.html"))
            .service(fs::Files::new("/", "./static/").index_file("index.html"))
            
            //.service()
            .default_service(web::route().to(|| HttpResponse::NotFound()))
        // .resource("/test", |r| r.f(welcome))
        // .resource("/static/{res}", |r| r.f(static_data))
        // .resource("/req/{name}", |r| r.f(greet))
    })
    .bind(("localhost", port))
    .expect("Can not bind to port 8000")
    .run();
}
