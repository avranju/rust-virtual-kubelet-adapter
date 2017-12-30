extern crate bodyparser;
extern crate iron;
extern crate kube_rust;
extern crate num;
extern crate persistent;
#[macro_use]
extern crate router;
extern crate serde;
extern crate serde_json;

mod provider;
pub use self::provider::Provider;

mod result;
pub use self::result::Result;

mod error;
pub use self::error::Error;

mod server;
pub use self::server::start_server;

mod utils;
mod routes;
mod types;
