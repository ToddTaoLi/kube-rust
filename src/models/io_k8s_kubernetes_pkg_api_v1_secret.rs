/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1Secret : Secret holds secret data of a certain type. The total bytes of the values in the Data field must be less than MaxSecretSize bytes.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1Secret {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// Data contains the secret data. Each key must consist of alphanumeric characters, '-', '_' or '.'. The serialized form of the secret data is a base64 encoded string, representing the arbitrary (possibly non-string) data value here. Described in https://tools.ietf.org/html/rfc4648#section-4
  #[serde(rename = "data")]
  data: Option<::std::collections::HashMap<String, String>>,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
  #[serde(rename = "metadata")]
  metadata: Option<::models::MetaV1ObjectMeta>,
  /// stringData allows specifying non-binary secret data in string form. It is provided as a write-only convenience method. All keys and values are merged into the data field on write, overwriting any existing values. It is never output when reading from the API.
  #[serde(rename = "stringData")]
  string_data: Option<::std::collections::HashMap<String, String>>,
  /// Used to facilitate programmatic handling of secret data.
  #[serde(rename = "type")]
  _type: Option<String>
}

impl V1Secret {
  /// Secret holds secret data of a certain type. The total bytes of the values in the Data field must be less than MaxSecretSize bytes.
  pub fn new() -> V1Secret {
    V1Secret {
      api_version: None,
      data: None,
      kind: None,
      metadata: None,
      string_data: None,
      _type: None
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> V1Secret {
    self.api_version = Some(api_version);
    self
  }

  pub fn api_version(&self) -> Option<&String> {
    self.api_version.as_ref()
  }

  pub fn reset_api_version(&mut self) {
    self.api_version = None;
  }

  pub fn set_data(&mut self, data: ::std::collections::HashMap<String, String>) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: ::std::collections::HashMap<String, String>) -> V1Secret {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&::std::collections::HashMap<String, String>> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

  pub fn set_kind(&mut self, kind: String) {
    self.kind = Some(kind);
  }

  pub fn with_kind(mut self, kind: String) -> V1Secret {
    self.kind = Some(kind);
    self
  }

  pub fn kind(&self) -> Option<&String> {
    self.kind.as_ref()
  }

  pub fn reset_kind(&mut self) {
    self.kind = None;
  }

  pub fn set_metadata(&mut self, metadata: ::models::MetaV1ObjectMeta) {
    self.metadata = Some(metadata);
  }

  pub fn with_metadata(mut self, metadata: ::models::MetaV1ObjectMeta) -> V1Secret {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::MetaV1ObjectMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_string_data(&mut self, string_data: ::std::collections::HashMap<String, String>) {
    self.string_data = Some(string_data);
  }

  pub fn with_string_data(mut self, string_data: ::std::collections::HashMap<String, String>) -> V1Secret {
    self.string_data = Some(string_data);
    self
  }

  pub fn string_data(&self) -> Option<&::std::collections::HashMap<String, String>> {
    self.string_data.as_ref()
  }

  pub fn reset_string_data(&mut self) {
    self.string_data = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> V1Secret {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

}



