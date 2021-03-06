/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RbacV1alpha1RoleRef : RoleRef contains information that points to the role being used

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RbacV1alpha1RoleRef {
  /// APIGroup is the group for the resource being referenced
  #[serde(rename = "apiGroup")]
  api_group: String,
  /// Kind is the type of resource being referenced
  #[serde(rename = "kind")]
  kind: String,
  /// Name is the name of resource being referenced
  #[serde(rename = "name")]
  name: String
}

impl RbacV1alpha1RoleRef {
  /// RoleRef contains information that points to the role being used
  pub fn new(api_group: String, kind: String, name: String) -> RbacV1alpha1RoleRef {
    RbacV1alpha1RoleRef {
      api_group: api_group,
      kind: kind,
      name: name
    }
  }

  pub fn set_api_group(&mut self, api_group: String) {
    self.api_group = api_group;
  }

  pub fn with_api_group(mut self, api_group: String) -> RbacV1alpha1RoleRef {
    self.api_group = api_group;
    self
  }

  pub fn api_group(&self) -> &String {
    &self.api_group
  }


  pub fn set_kind(&mut self, kind: String) {
    self.kind = kind;
  }

  pub fn with_kind(mut self, kind: String) -> RbacV1alpha1RoleRef {
    self.kind = kind;
    self
  }

  pub fn kind(&self) -> &String {
    &self.kind
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> RbacV1alpha1RoleRef {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


}



