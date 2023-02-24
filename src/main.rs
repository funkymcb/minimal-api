#[macro_use]
extern crate rocket;
use rocket::get;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::serde::json::{json, Value};
use rocket::fairing::{Fairing, Info, Kind};

use std::env;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/", format = "json")]
fn get() -> Value {
    let msg = env::var("API_MESSAGE").expect("$API_MESSAGE is not set");
    json!({
        "message": msg,
    })
}

#[get("/live", format = "json")]
fn live() -> Value {
    json!({
        "message": "API is alive",
    })
}

#[get("/health", format = "json")]
fn health() -> Value {
    json!({
        "message": "API is healthy",
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get, live, health]).attach(CORS)
}
