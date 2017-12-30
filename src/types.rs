use iron::typemap::Key;
use provider::Provider;

#[derive(Clone)]
pub struct ProviderState<T: Provider + 'static> {
    pub prov: Box<T>,
}

impl<T: Provider> Key for ProviderState<T> {
    type Value = ProviderState<T>;
}
