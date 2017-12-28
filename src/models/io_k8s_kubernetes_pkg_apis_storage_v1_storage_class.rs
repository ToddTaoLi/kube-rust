/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StorageV1StorageClass : StorageClass describes the parameters for a class of storage for which PersistentVolumes can be dynamically provisioned.  StorageClasses are non-namespaced; the name of the storage class according to etcd is in ObjectMeta.Name.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorageV1StorageClass {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
  #[serde(rename = "metadata")]
  metadata: Option<::models::MetaV1ObjectMeta>,
  /// Parameters holds the parameters for the provisioner that should create volumes of this storage class.
  #[serde(rename = "parameters")]
  parameters: Option<::std::collections::HashMap<String, String>>,
  /// Provisioner indicates the type of the provisioner.
  #[serde(rename = "provisioner")]
  provisioner: String
}

impl StorageV1StorageClass {
  /// StorageClass describes the parameters for a class of storage for which PersistentVolumes can be dynamically provisioned.  StorageClasses are non-namespaced; the name of the storage class according to etcd is in ObjectMeta.Name.
  pub fn new(provisioner: String) -> StorageV1StorageClass {
    StorageV1StorageClass {
      api_version: None,
      kind: None,
      metadata: None,
      parameters: None,
      provisioner: provisioner
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> StorageV1StorageClass {
    self.api_version = Some(api_version);
    self
  }

  pub fn api_version(&self) -> Option<&String> {
    self.api_version.as_ref()
  }

  pub fn reset_api_version(&mut self) {
    self.api_version = None;
  }

  pub fn set_kind(&mut self, kind: String) {
    self.kind = Some(kind);
  }

  pub fn with_kind(mut self, kind: String) -> StorageV1StorageClass {
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

  pub fn with_metadata(mut self, metadata: ::models::MetaV1ObjectMeta) -> StorageV1StorageClass {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::MetaV1ObjectMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_parameters(&mut self, parameters: ::std::collections::HashMap<String, String>) {
    self.parameters = Some(parameters);
  }

  pub fn with_parameters(mut self, parameters: ::std::collections::HashMap<String, String>) -> StorageV1StorageClass {
    self.parameters = Some(parameters);
    self
  }

  pub fn parameters(&self) -> Option<&::std::collections::HashMap<String, String>> {
    self.parameters.as_ref()
  }

  pub fn reset_parameters(&mut self) {
    self.parameters = None;
  }

  pub fn set_provisioner(&mut self, provisioner: String) {
    self.provisioner = provisioner;
  }

  pub fn with_provisioner(mut self, provisioner: String) -> StorageV1StorageClass {
    self.provisioner = provisioner;
    self
  }

  pub fn provisioner(&self) -> &String {
    &self.provisioner
  }


}



