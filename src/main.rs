#[macro_use]
extern crate rocket;
use rocket::get;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::serde::json::{json, Value};
use rocket::fairing::{Fairing, Info, Kind};

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
    json!({
        "message": "API is alive",
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![get]).attach(CORS)
}
