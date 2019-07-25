use http_guest::{Request, Response, guest_app};
use comrak::{markdown_to_html, ComrakOptions};

pub fn user_entrypoint(_req: &Request<Vec<u8>>) -> Response<Vec<u8>> {
    let markdown = r#"
Sample Markdown
===

This is a sample Markdown document, and here's a link to [Google](https://google.com).
"#;

    let html = markdown_to_html(markdown, &ComrakOptions::default());
    Response::builder()
        .status(200)
        .header("content-type", "text/html; charset=utf-8")
        .body(html.as_bytes().to_owned())
        .unwrap()
}

guest_app!(user_entrypoint);
