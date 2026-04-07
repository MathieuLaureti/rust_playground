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

pub async fn serialize_to_bytes<T: serde::Serialize + Send + 'static>(data: T, default: &'static [u8]) -> Bytes {
    tokio::task::spawn_blocking(move || {
        serde_json::to_vec(&data)
            .map(Bytes::from)
            .unwrap_or_else(|_| Bytes::copy_from_slice(default))
    })
    .await
    .unwrap_or_else(|_| Bytes::copy_from_slice(default))
}
