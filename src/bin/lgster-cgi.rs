use std::error::Error;
use cgi::handle;
use http::header;
use http::response;
use http::request;

fn create_response<S>(content_type: &str, body: S) -> response::Response<Vec<u8>>
    where S: Into<String>
{
    let body: Vec<u8> = body.into().into_bytes();
    response::Builder::new()
        .status(200)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_LENGTH, format!("{}", body.len()).as_str())
        .body(body)
        .unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(handle(|_request: request::Request<Vec<u8>>| -> response::Response<Vec<u8>> {
        create_response("application/json; charset=utf-8", "{}")
    }))
}
