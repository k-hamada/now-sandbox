use http::{self, Request, Response, StatusCode};

fn handler(request: Request<()>) -> http::Result<Response<String>> {
    Response::builder()
        .status(StatusCode::OK)
        .body(request.uri().path().to_string())
}
