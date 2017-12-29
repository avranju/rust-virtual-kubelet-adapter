use iron::prelude::*;
use persistent::Read;
use router::Router;

use provider::Provider;
use result::Result;
use routes;
use utils;
use types::ProviderState;

pub fn start_server<T>(prov: Box<T>) -> Result<()>
where
    T: Provider + 'static + Send + Sync,
{
    // create a chain with a route map
    let mut chain = Chain::new(setup_route_map::<T>());

    let provider_state = ProviderState { prov };

    // this object manages thread-safe access to the shared provider state
    let safe_provider_state = Read::<ProviderState<T>>::one(provider_state);

    // add a "before" middleware for injecting our provider state
    chain.link_before(safe_provider_state);

    // start the web server
    let port = utils::get_env_integral("PORT", Ok(3000u16));
    Iron::new(chain).http(format!("0.0.0.0:{}", port)).unwrap();

    Ok(())
}

fn setup_route_map<T>() -> Router
where
    T: Provider + 'static + Send + Sync,
{
    router!(
    index: get "/" => routes::default,
    create_pod: post "/createPod" => routes::create_pod::<T>,
    update_pod: put "/updatePod" => routes::update_pod
  )
}
