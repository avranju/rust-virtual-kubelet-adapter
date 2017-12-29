use iron::prelude::*;
use num;
use persistent::Read;
use std::sync::Arc;
use provider::Provider;
use types::ProviderState;

pub fn get_env_integral<T>(var_name: &str, default_value: Result<T, T>) -> T
where
    T: num::Integer,
{
    use std::env;
    let val = match env::var(var_name) {
        Ok(val) => T::from_str_radix(&val, 10).or(default_value),
        Err(_) => default_value,
    };
    val.ok().unwrap()
}

pub fn get_provider<T>(req: &mut Request) -> Arc<ProviderState<T>>
where
    T: Provider + 'static + Send + Sync,
{
    // we want to panic if this fails because the provider *SHOULD* have been
    // injected from "start_server"
    req.get::<Read<ProviderState<T>>>().unwrap()
}
