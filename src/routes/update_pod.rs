use iron::prelude::*;
use iron::status;

pub fn update_pod(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Pod updated")))
}
