use kube_rust::models::{V1NodeCondition, V1Pod, V1PodStatus, V1NodeAddress, V1NodeDaemonEndpoints};
use std::collections::BTreeMap;

use result::Result;

pub trait Provider {
    fn create_pod(&mut self, pod: &V1Pod) -> Result<()>;

    fn update_pod(&mut self, pod: &V1Pod) -> Result<()>;

    fn delete_pod(&mut self, pod: &V1Pod) -> Result<()>;

    fn get_pod(&self, namespace: &str, name: &str) -> Result<V1Pod>;

    fn get_container_logs(&self, namespace: &str, pod_name: &str, container_name: &str, tail: i32) -> Result<String>;

    fn get_pod_status(&self, namespace: &str, name: &str) -> Result<V1PodStatus>;

    fn get_pods(&self) -> Result<Vec<V1Pod>>;

    fn capacity(&self) -> Result<BTreeMap<String, String>>;

    fn node_conditions(&self) -> Result<Vec<V1NodeCondition>>;

    fn node_addresses(&self) -> Result<Vec<V1NodeAddress>>;

    fn node_daemon_endpoints(&self) -> Result<V1NodeDaemonEndpoints>;

    fn operating_system(&self) -> String;
}
