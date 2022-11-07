use std::convert::Infallible;
use rocket::request::{FromRequest, Outcome};
use rocket::http::HeaderMap;
use rocket::{Request, Config};
use rocket::serde::json::serde_json::{json, self};
use dotenv::dotenv;

#[macro_use] extern crate rocket;


#[get("/ping")]
fn ping(headers: RequestHeaders) -> serde_json::Value {
    let mut res = json!({});
    for header in headers.0.iter()
    {
        res.as_object_mut().unwrap().insert(header.name().to_string(), json!(header.value().to_string()));
    }

    res
}


#[catch(500)]
fn internal_error() -> serde_json::Value {
    json!({})
}

#[catch(404)]
fn not_found(req: &Request) -> serde_json::Value {
    json!({})
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let port = std::env::var("PING_LISTEN_PORT");
    let config = Config {
        address : "0.0.0.0".parse().unwrap(),
        port: port.unwrap_or("8080".to_string()).parse().unwrap(),
        ..Config::debug_default()
    };
    rocket::custom(&config).mount("/", routes![ping]).register("/",  catchers![internal_error, not_found])
}

struct RequestHeaders<'h>(&'h HeaderMap<'h>);
#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestHeaders<'r> {
    type Error = Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let request_headers = request.headers();
        Outcome::Success(RequestHeaders(request_headers))
    }
}


