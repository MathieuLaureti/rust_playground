use hyper::{Response};
use http_body_util::Full;
use hyper::body::Bytes;

pub fn build_response(status: u16, body_text: &'static str) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .header("Content-Type", "text/plain") // Optional: good practice
        .body(Full::new(Bytes::from(body_text)))
        .unwrap() // Safe here if you know status and body are valid
}