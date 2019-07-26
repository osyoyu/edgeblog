use failure::{Error, format_err};
use http_guest::{PendingRequest, Request, RequestExt};

pub struct S3Request {
    location: String,
    pending: PendingRequest,
}

impl S3Request {
    pub fn new(location: &str) -> Result<S3Request, Error> {
        // Build a request
        let req = Request::builder()
            .method("GET")
            .header("accept", "application/json")
            .uri(format!(
                "http://osyoyu-edgeblog.s3-website-ap-northeast-1.amazonaws.com/{}",
                location
            ))
            .body(vec![])?;

        // Initiate the request in async
        let pending = req.send_async()?;
        Ok(S3Request {
            location: location.to_owned(),
            pending: pending,
        })
    }

    pub fn get_response(self) -> Result<String, Error> {
        // Block for the response
        let res = self.pending.wait()?;

        if res.status() == 200 {
            Ok(std::str::from_utf8(res.body()).unwrap_or("(utf8 error)").to_string())
        }
        else {
            let err_body = std::str::from_utf8(res.body()).unwrap_or("(utf8 error)");
            Err(format_err!(
                "S3 request for {} returned {}: {}",
                self.location,
                res.status(),
                err_body,
            ))
        }
    }
}
