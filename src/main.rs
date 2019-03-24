use http::{self, StatusCode};
use now_lambda::{lambda, IntoResponse, Request, Response};
use std::error::Error;

fn handler(request: Request) -> Result<impl IntoResponse, http::Error> {
    match request.uri().path() {
        "/today" => today(),
        _ => not_found()
    }
}

fn today() -> http::Result<Response<String>> {
    Response::builder()
        .status(StatusCode::OK)
        .body("today".to_string())
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
