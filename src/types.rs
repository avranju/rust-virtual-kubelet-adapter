use iron::typemap::Key;
use provider::Provider;

#[derive(Clone)]
pub struct ProviderState<T: Provider> {
    pub prov: Box<T>,
}

impl<T: Provider + 'static> Key for ProviderState<T> {
    type Value = ProviderState<T>;
}
