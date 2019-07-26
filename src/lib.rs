mod s3_request;

use http_guest::{Request, Response, guest_app};
use comrak::{markdown_to_html, ComrakOptions};

use s3_request::S3Request;


pub fn user_entrypoint(_req: &Request<Vec<u8>>) -> Response<Vec<u8>> {
    let s3_request = S3Request::new("sample.md").unwrap();
    let markdown = s3_request.get_response().unwrap_or("error".to_string());

    let html = markdown_to_html(&markdown, &ComrakOptions::default());
    Response::builder()
        .status(200)
        .header("content-type", "text/html; charset=utf-8")
        .body(html.as_bytes().to_owned())
        .unwrap()
}

guest_app!(user_entrypoint);
