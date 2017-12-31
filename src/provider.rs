use kube_rust::models::{V1NodeCondition, V1Pod, V1PodStatus};
use std::collections::BTreeMap;

use result::Result;

pub trait Provider {
    fn create_pod(&mut self, pod: &V1Pod) -> Result<()>;

    fn update_pod(&mut self, pod: &V1Pod) -> Result<()>;

    fn delete_pod(&mut self, pod: &V1Pod) -> Result<()>;

    fn get_pod(&self, namespace: &str, name: &str) -> Result<V1Pod>;

    fn get_pod_status(&self, namespace: &str, name: &str) -> Result<V1PodStatus>;

    fn get_pods(&self) -> Result<Vec<V1Pod>>;

    fn capacity(&self) -> Result<BTreeMap<String, String>>;

    fn node_conditions(&self) -> Result<Vec<V1NodeCondition>>;

    fn operating_system(&self) -> String;
}
