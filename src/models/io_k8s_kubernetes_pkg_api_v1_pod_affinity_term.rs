/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1PodAffinityTerm : Defines a set of pods (namely those matching the labelSelector relative to the given namespace(s)) that this pod should be co-located (affinity) or not co-located (anti-affinity) with, where co-located is defined as running on a node whose value of the label with key <topologyKey> tches that of any node on which a pod of the set of pods is running

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1PodAffinityTerm {
  /// A label query over a set of resources, in this case pods.
  #[serde(rename = "labelSelector")]
  label_selector: Option<::models::MetaV1LabelSelector>,
  /// namespaces specifies which namespaces the labelSelector applies to (matches against); null or empty list means \"this pod's namespace\"
  #[serde(rename = "namespaces")]
  namespaces: Option<Vec<String>>,
  /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. For PreferredDuringScheduling pod anti-affinity, empty topologyKey is interpreted as \"all topologies\" (\"all topologies\" here means all the topologyKeys indicated by scheduler command-line argument --failure-domains); for affinity and for RequiredDuringScheduling pod anti-affinity, empty topologyKey is not allowed.
  #[serde(rename = "topologyKey")]
  topology_key: Option<String>
}

impl V1PodAffinityTerm {
  /// Defines a set of pods (namely those matching the labelSelector relative to the given namespace(s)) that this pod should be co-located (affinity) or not co-located (anti-affinity) with, where co-located is defined as running on a node whose value of the label with key <topologyKey> tches that of any node on which a pod of the set of pods is running
  pub fn new() -> V1PodAffinityTerm {
    V1PodAffinityTerm {
      label_selector: None,
      namespaces: None,
      topology_key: None
    }
  }

  pub fn set_label_selector(&mut self, label_selector: ::models::MetaV1LabelSelector) {
    self.label_selector = Some(label_selector);
  }

  pub fn with_label_selector(mut self, label_selector: ::models::MetaV1LabelSelector) -> V1PodAffinityTerm {
    self.label_selector = Some(label_selector);
    self
  }

  pub fn label_selector(&self) -> Option<&::models::MetaV1LabelSelector> {
    self.label_selector.as_ref()
  }

  pub fn reset_label_selector(&mut self) {
    self.label_selector = None;
  }

  pub fn set_namespaces(&mut self, namespaces: Vec<String>) {
    self.namespaces = Some(namespaces);
  }

  pub fn with_namespaces(mut self, namespaces: Vec<String>) -> V1PodAffinityTerm {
    self.namespaces = Some(namespaces);
    self
  }

  pub fn namespaces(&self) -> Option<&Vec<String>> {
    self.namespaces.as_ref()
  }

  pub fn reset_namespaces(&mut self) {
    self.namespaces = None;
  }

  pub fn set_topology_key(&mut self, topology_key: String) {
    self.topology_key = Some(topology_key);
  }

  pub fn with_topology_key(mut self, topology_key: String) -> V1PodAffinityTerm {
    self.topology_key = Some(topology_key);
    self
  }

  pub fn topology_key(&self) -> Option<&String> {
    self.topology_key.as_ref()
  }

  pub fn reset_topology_key(&mut self) {
    self.topology_key = None;
  }

}



