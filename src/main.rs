use http::{self, StatusCode};
use now_lambda::{lambda, IntoResponse, Request, Response};
use std::error::Error;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct JsonToday {
    today: bool,
}

fn handler(request: Request) -> Result<impl IntoResponse, http::Error> {
    let (status_code, body) = match request.uri().path() {
        "/today" => today(),
        "/json" => json(),
        _ => not_found()
    };

    Response::builder()
        .status(status_code)
        .body(body)
}

fn today() -> (StatusCode, String) {
    (StatusCode::OK, "today".to_string())
}

fn json() -> (StatusCode, String) {
    let value = JsonToday { today: true };
    let serialized = serde_json::to_string(&value);
    match serialized {
        Ok(json) => (StatusCode::OK, json),
        Err(err) =>(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }
}

fn not_found() -> (StatusCode, String) {
    let usage = "\
        USAGE\
        \n    GET /<year>/<month>/<day>\
        \n    GET /today\
        \n    GET /json\
    ";

    (StatusCode::OK, usage.to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
