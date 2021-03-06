/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1TcpSocketAction : TCPSocketAction describes an action based on opening a socket

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1TcpSocketAction {
  /// Optional: Host name to connect to, defaults to the pod IP.
  #[serde(rename = "host")]
  host: Option<String>,
  /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
  #[serde(rename = "port")]
  port: ::models::UtilIntstrIntOrString
}

impl V1TcpSocketAction {
  /// TCPSocketAction describes an action based on opening a socket
  pub fn new(port: ::models::UtilIntstrIntOrString) -> V1TcpSocketAction {
    V1TcpSocketAction {
      host: None,
      port: port
    }
  }

  pub fn set_host(&mut self, host: String) {
    self.host = Some(host);
  }

  pub fn with_host(mut self, host: String) -> V1TcpSocketAction {
    self.host = Some(host);
    self
  }

  pub fn host(&self) -> Option<&String> {
    self.host.as_ref()
  }

  pub fn reset_host(&mut self) {
    self.host = None;
  }

  pub fn set_port(&mut self, port: ::models::UtilIntstrIntOrString) {
    self.port = port;
  }

  pub fn with_port(mut self, port: ::models::UtilIntstrIntOrString) -> V1TcpSocketAction {
    self.port = port;
    self
  }

  pub fn port(&self) -> &::models::UtilIntstrIntOrString {
    &self.port
  }


}



