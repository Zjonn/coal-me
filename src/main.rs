use chrono;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use actix_diesel::Database;
use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};

mod auth;

#[macro_use]
extern crate lazy_static;

lazy_static::lazy_static! {
pub  static ref SECRET_KEY: String = "CoalME".repeat(8);
}

pub struct AppState {
    db: PgConnection,
}

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Get the port number to listen on.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    // Start a server, configuring the resources to serve.
    HttpServer::new(move || {
        App::new()
            // .data(AppState {
            //     db: establish_connection(),
            // })
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .secure(true)
                    .domain("https://coal-me.herokuapp.com/")
                    .max_age_time(chrono::Duration::hours(12)),
            ))
            .service(fs::Files::new("/", "./static").index_file("index.html"))
            .service(
                web::resource("/auth/{userID}")
                    .route(web::post().to(auth::login))
                    .route(web::get().to(auth::logged))
                    .route(web::delete().to(auth::logout)),
            )
            .service(web::scope("/api").service(web::resource("material").route(web::get())))
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
    .bind(("localhost", port))
    .expect("Can not bind to port 8000")
    .run()
    .await
}
