use http_guest::{Request, Response};
use lazy_static::lazy_static;
use regex::Regex;
use comrak::{markdown_to_html, ComrakOptions};

use crate::s3_request::S3Request;

pub fn user_entrypoint(req: &Request<Vec<u8>>) -> Response<Vec<u8>> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new("/posts/(.+)").expect("regex");
    }

    let response_body = match RE.captures(req.uri().path()) {
        Some(captures) => {
            let filename = captures.get(1).unwrap().as_str();
            let s3_request = S3Request::new(&format!("{}.md", filename)).unwrap();
            let markdown = s3_request.get_response().unwrap_or("error".to_string());
            let html = markdown_to_html(&markdown, &ComrakOptions::default());

            html
        }
        _ => {
            // Return early with a 404
            return Response::builder().status(404).body(vec![]).unwrap();
        }
    };

    Response::builder()
        .status(200)
        .header("content-type", "text/html; charset=utf-8")
        .body(response_body.as_bytes().to_owned())
        .unwrap()
}
