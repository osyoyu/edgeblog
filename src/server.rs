use http_guest::{Request, Response};
use lazy_static::lazy_static;
use regex::Regex;

use crate::actions;

pub fn user_entrypoint(req: &Request<Vec<u8>>) -> Response<Vec<u8>> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new("/(.+)/(.+)").expect("regex");
    }

    let response_body = match RE.captures(req.uri().path()) {
        // route requests
        Some(captures) => {
            match captures.get(1).unwrap().as_str() {
                "posts" => {
                    actions::post_show(captures.get(2).unwrap().as_str().to_string())
                },
                "search" => {
                    actions::post_show(captures.get(2).unwrap().as_str().to_string())
                },
                _ => {
                    // Return early with a 404
                    return Response::builder().status(404).body(vec![]).unwrap();
                }
            }
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
