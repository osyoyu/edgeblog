use comrak::{markdown_to_html, ComrakOptions};

use crate::s3_request::S3Request;

pub fn post_show(label: String) -> String {
    let s3_request = S3Request::new(&format!("{}.md", label)).unwrap();
    let markdown = s3_request.get_response().unwrap_or("error".to_string());
    markdown_to_html(&markdown, &ComrakOptions::default())
}
