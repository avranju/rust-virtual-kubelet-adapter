use iron::prelude::*;
use iron::status;

pub fn default(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
        status::Ok,
        "Welcome to Rust kubelet provider",
    )))
}
