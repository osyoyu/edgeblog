mod server;
mod s3_request;

use http_guest::guest_app;
use crate::server::user_entrypoint;

// The entry point of this app
guest_app!(user_entrypoint);
