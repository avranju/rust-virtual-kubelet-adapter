use iron::prelude::*;
use iron::status;
use kube_rust::models::V1Pod;
use bodyparser;
use utils::get_provider;
use provider::Provider;

pub fn create_pod<T>(req: &mut Request) -> IronResult<Response>
where
    T: Provider + 'static + Send + Sync,
{
    let pod_body = req.get::<bodyparser::Struct<V1Pod>>();
    let result = match pod_body {
        Ok(Some(pod)) => {
            let provider = get_provider::<T>(req);

            match provider.prov.create_pod(&pod) {
                Ok(()) => (status::Ok, "Pod created".to_owned()),
                Err(err) => (status::BadRequest, err.message),
            }
        }
        Ok(None) => (
            status::BadRequest,
            "Empty pod specification received".to_owned(),
        ),
        Err(_) => (
            status::BadRequest,
            "Invalid pod specification received".to_owned(),
        ),
    };

    Ok(Response::with(result))
}
