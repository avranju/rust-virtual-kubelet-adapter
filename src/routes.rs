use router::Router;
use utils::get_provider;
use kube_rust::models::V1Pod;
use types::ProviderState;
use std::sync::Arc;
use provider::Provider;
use iron::prelude::*;
use iron::status;
use bodyparser;
use serde_json;
use serde::Serialize;

pub fn default(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
        status::Ok,
        "Welcome to Rust kubelet provider",
    )))
}

pub fn create_pod<T>(req: &mut Request) -> IronResult<Response>
where
    T: Provider + 'static + Send + Sync,
{
    // The sequence of combinators here does the following:
    //  [1] If get_provider_pod returns an Err((status, string)) then that gets
    //      directly assigned to "result".
    //  [2] If get_provider_pod returns an Ok((provider, pod)) then that gets
    //      sent to the first 'map' closure which sends it along to the
    //      'create_pod' function. This function returns a Result<(), Error>.
    //  [3] The result of calling 'create_pod' is sent to the 'and_then'
    //      combinator which handles the error case because that's the only
    //      interesting option here (the success case is an empty tuple). It
    //      handles it by transforming it into a (status, String) tuple.
    //  [4] The result of 'and_then' is then converted to an Option<E> via the
    //      call to the 'err()' combinator which produces a Some(E) if the
    //      result was an error or a None.
    //  [5] Finally, we transform the Option<E> into a (status, String) pair
    //      using the "default" value of 'map_or' for the success case which
    //      would be used if 'create_pod' was successful and the error value
    //      as-is in case it failed.
    let result = get_provider_pod::<T>(req)
        .map(|(provider, pod)| provider.prov.create_pod(&pod))
        .and_then(|result| result.map_err(|err| (status::InternalServerError, err.message)))
        .err()
        .map_or((status::Ok, "Pod created".to_string()), |e| e);

    Ok(Response::with(result))
}

pub fn update_pod<T>(req: &mut Request) -> IronResult<Response>
where
    T: Provider + 'static + Send + Sync,
{
    // See comments in "create_pod" to help understand what this line does.
    let result = get_provider_pod::<T>(req)
        .map(|(provider, pod)| provider.prov.update_pod(&pod))
        .and_then(|result| result.map_err(|err| (status::InternalServerError, err.message)))
        .err()
        .map_or((status::Ok, "Pod updated".to_string()), |e| e);

    Ok(Response::with(result))
}

fn get_provider_pod<T>(
    req: &mut Request,
) -> Result<(Arc<ProviderState<T>>, V1Pod), (status::Status, String)>
where
    T: Provider + 'static + Send + Sync,
{
    let pod_body = req.get::<bodyparser::Struct<V1Pod>>();
    match pod_body {
        Ok(Some(pod)) => {
            let provider = get_provider::<T>(req);
            Ok((provider, pod))
        }
        Ok(None) => Err((
            status::BadRequest,
            "Empty pod specification received".to_string(),
        )),
        Err(_) => Err((
            status::BadRequest,
            "Invalid pod specification received".to_string(),
        )),
    }
}

pub fn delete_pod<T>(req: &mut Request) -> IronResult<Response>
where
    T: Provider + 'static + Send + Sync,
{
    // See comments in "create_pod" to help understand what this line does.
    let result = get_provider_pod::<T>(req)
        .map(|(provider, pod)| provider.prov.delete_pod(&pod))
        .and_then(|result| result.map_err(|err| (status::InternalServerError, err.message)))
        .err()
        .map_or((status::Ok, "Pod deleted".to_string()), |e| e);

    Ok(Response::with(result))
}

pub fn get_pod<T>(req: &mut Request) -> IronResult<Response>
where
    T: Provider + 'static + Send + Sync,
{
    get_pod_helper(req, |provider: Arc<ProviderState<T>>, ns, name| {
        provider.prov.get_pod(ns, name)
    })
}

pub fn get_pod_status<T>(req: &mut Request) -> IronResult<Response>
where
    T: Provider + 'static + Send + Sync,
{
    get_pod_helper(req, |provider: Arc<ProviderState<T>>, ns, name| {
        provider.prov.get_pod_status(ns, name)
    })
}

pub fn get_pods<T>(req: &mut Request) -> IronResult<Response>
where
    T: Provider + 'static + Send + Sync,
{
    get_pod_helper(req, |provider: Arc<ProviderState<T>>, _, _| {
        provider.prov.get_pods()
    })
}

pub fn capacity<T>(req: &mut Request) -> IronResult<Response>
where
    T: Provider + 'static + Send + Sync,
{
    get_pod_helper(req, |provider: Arc<ProviderState<T>>, _, _| {
        provider.prov.capacity()
    })
}

pub fn node_conditions<T>(req: &mut Request) -> IronResult<Response>
where
    T: Provider + 'static + Send + Sync,
{
    get_pod_helper(req, |provider: Arc<ProviderState<T>>, _, _| {
        provider.prov.node_conditions()
    })
}

fn get_pod_helper<T, F, P>(req: &mut Request, get_data: F) -> IronResult<Response>
where
    T: Provider + 'static + Send + Sync,
    F: Fn(Arc<ProviderState<T>>, &str, &str) -> ::result::Result<P>,
    P: Serialize,
{
    let provider = get_provider::<T>(req);
    let params = req.extensions.get::<Router>().unwrap();
    let ns = params.find("namespace").map_or("", |n| n);
    let name = params.find("name").map_or("", |n| n);

    let result = match get_data(provider, ns, name) {
        Ok(data) => serde_json::to_string(&data)
            .map_err(|e| (status::InternalServerError, format!("{}", e))),
        Err(err) => Err((status::InternalServerError, err.message)),
    };

    match result {
        Ok(json) => Ok(Response::with((status::Ok, json))),
        Err(err) => Ok(Response::with(err)),
    }
}

pub fn operating_system<T>(req: &mut Request) -> IronResult<Response>
where
    T: Provider + 'static + Send + Sync,
{
    Ok(Response::with((
        status::Ok,
        get_provider::<T>(req).prov.operating_system(),
    )))
}