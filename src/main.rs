use http::{self, StatusCode};
use now_lambda::{lambda, IntoResponse, Request, Response};
use std::error::Error;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct JsonToday {
    today: bool,
}

fn handler(request: Request) -> Result<impl IntoResponse, http::Error> {
    match request.uri().path() {
        "/today" => today(),
        "/json" => json(),
        _ => not_found()
    }
}

fn today() -> http::Result<Response<String>> {
    Response::builder()
        .status(StatusCode::OK)
        .body("today".to_string())
}

fn json() -> http::Result<Response<String>> {
    let value = JsonToday { today: true };
    let serialized = serde_json::to_string(&value);
    let (status_code, body) = match serialized {
        Ok(json) => (StatusCode::OK, json),
        Err(err) =>(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    };

    Response::builder()
        .status(status_code)
        .body(body)
}

fn not_found() -> http::Result<Response<String>> {
    let usage = "\
        USAGE\
        \n    GET /<year>/<month>/<day>\
        \n    GET /today\
        \n    GET /json\
    ";
    Response::builder()
        .status(StatusCode::OK)
        .body(usage.to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
