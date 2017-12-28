/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// MetaV1ServerAddressByClientCidr : ServerAddressByClientCIDR helps the client to determine the server address that they should use, depending on the clientCIDR that they match.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaV1ServerAddressByClientCidr {
  /// The CIDR with which clients can match their IP to figure out the server address that they should use.
  #[serde(rename = "clientCIDR")]
  client_cidr: String,
  /// Address of this server, suitable for a client that matches the above CIDR. This can be a hostname, hostname:port, IP or IP:port.
  #[serde(rename = "serverAddress")]
  server_address: String
}

impl MetaV1ServerAddressByClientCidr {
  /// ServerAddressByClientCIDR helps the client to determine the server address that they should use, depending on the clientCIDR that they match.
  pub fn new(client_cidr: String, server_address: String) -> MetaV1ServerAddressByClientCidr {
    MetaV1ServerAddressByClientCidr {
      client_cidr: client_cidr,
      server_address: server_address
    }
  }

  pub fn set_client_cidr(&mut self, client_cidr: String) {
    self.client_cidr = client_cidr;
  }

  pub fn with_client_cidr(mut self, client_cidr: String) -> MetaV1ServerAddressByClientCidr {
    self.client_cidr = client_cidr;
    self
  }

  pub fn client_cidr(&self) -> &String {
    &self.client_cidr
  }


  pub fn set_server_address(&mut self, server_address: String) {
    self.server_address = server_address;
  }

  pub fn with_server_address(mut self, server_address: String) -> MetaV1ServerAddressByClientCidr {
    self.server_address = server_address;
    self
  }

  pub fn server_address(&self) -> &String {
    &self.server_address
  }


}



