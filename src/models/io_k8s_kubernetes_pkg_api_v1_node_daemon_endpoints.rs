/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1NodeDaemonEndpoints : NodeDaemonEndpoints lists ports opened by daemons running on the Node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1NodeDaemonEndpoints {
  /// Endpoint on which Kubelet is listening.
  #[serde(rename = "kubeletEndpoint")]
  kubelet_endpoint: Option<::models::V1DaemonEndpoint>
}

impl V1NodeDaemonEndpoints {
  /// NodeDaemonEndpoints lists ports opened by daemons running on the Node.
  pub fn new() -> V1NodeDaemonEndpoints {
    V1NodeDaemonEndpoints {
      kubelet_endpoint: None
    }
  }

  pub fn set_kubelet_endpoint(&mut self, kubelet_endpoint: ::models::V1DaemonEndpoint) {
    self.kubelet_endpoint = Some(kubelet_endpoint);
  }

  pub fn with_kubelet_endpoint(mut self, kubelet_endpoint: ::models::V1DaemonEndpoint) -> V1NodeDaemonEndpoints {
    self.kubelet_endpoint = Some(kubelet_endpoint);
    self
  }

  pub fn kubelet_endpoint(&self) -> Option<&::models::V1DaemonEndpoint> {
    self.kubelet_endpoint.as_ref()
  }

  pub fn reset_kubelet_endpoint(&mut self) {
    self.kubelet_endpoint = None;
  }

}



