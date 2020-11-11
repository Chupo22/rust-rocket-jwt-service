#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate rocket_codegen;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

mod db;
mod handlers;
mod models;
mod schema;

use crate::handlers::login::LoginRequest;
use db::init_pool;
use handlers::login::login as login_handler;
use rocket_contrib::json::{Json, JsonValue};

#[get("/login")]
fn login(connection: db::Connection) -> Json<JsonValue> {
    let respones = login_handler(
        // TODO: Take from request
        LoginRequest::new("tmp".to_string(), "tmp".to_string()),
        &connection,
    );

    Json(json!(respones))
}

fn main() {
    rocket::ignite()
        .manage(init_pool())
        .mount("/auth", routes![login])
        .launch();
}
