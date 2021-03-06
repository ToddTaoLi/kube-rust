/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AutoscalingV2alpha1CrossVersionObjectReference : CrossVersionObjectReference contains enough information to let you identify the referred resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AutoscalingV2alpha1CrossVersionObjectReference {
  /// API version of the referent
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// Kind of the referent; More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds\"
  #[serde(rename = "kind")]
  kind: String,
  /// Name of the referent; More info: http://kubernetes.io/docs/user-guide/identifiers#names
  #[serde(rename = "name")]
  name: String
}

impl AutoscalingV2alpha1CrossVersionObjectReference {
  /// CrossVersionObjectReference contains enough information to let you identify the referred resource.
  pub fn new(kind: String, name: String) -> AutoscalingV2alpha1CrossVersionObjectReference {
    AutoscalingV2alpha1CrossVersionObjectReference {
      api_version: None,
      kind: kind,
      name: name
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> AutoscalingV2alpha1CrossVersionObjectReference {
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
    self.kind = kind;
  }

  pub fn with_kind(mut self, kind: String) -> AutoscalingV2alpha1CrossVersionObjectReference {
    self.kind = kind;
    self
  }

  pub fn kind(&self) -> &String {
    &self.kind
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> AutoscalingV2alpha1CrossVersionObjectReference {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


}



