/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RbacV1alpha1PolicyRule : PolicyRule holds information that describes a policy rule, but does not contain information about who the rule applies to or which namespace the rule applies to.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RbacV1alpha1PolicyRule {
  /// APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of the enumerated resources in any API group will be allowed.
  #[serde(rename = "apiGroups")]
  api_groups: Option<Vec<String>>,
  /// NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path This name is intentionally different than the internal type so that the DefaultConvert works nicely and because the ordering may be different. Since non-resource URLs are not namespaced, this field is only applicable for ClusterRoles referenced from a ClusterRoleBinding. Rules can either apply to API resources (such as \"pods\" or \"secrets\") or non-resource URL paths (such as \"/api\"),  but not both.
  #[serde(rename = "nonResourceURLs")]
  non_resource_ur_ls: Option<Vec<String>>,
  /// ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.
  #[serde(rename = "resourceNames")]
  resource_names: Option<Vec<String>>,
  /// Resources is a list of resources this rule applies to.  ResourceAll represents all resources.
  #[serde(rename = "resources")]
  resources: Option<Vec<String>>,
  /// Verbs is a list of Verbs that apply to ALL the ResourceKinds and AttributeRestrictions contained in this rule.  VerbAll represents all kinds.
  #[serde(rename = "verbs")]
  verbs: Vec<String>
}

impl RbacV1alpha1PolicyRule {
  /// PolicyRule holds information that describes a policy rule, but does not contain information about who the rule applies to or which namespace the rule applies to.
  pub fn new(verbs: Vec<String>) -> RbacV1alpha1PolicyRule {
    RbacV1alpha1PolicyRule {
      api_groups: None,
      non_resource_ur_ls: None,
      resource_names: None,
      resources: None,
      verbs: verbs
    }
  }

  pub fn set_api_groups(&mut self, api_groups: Vec<String>) {
    self.api_groups = Some(api_groups);
  }

  pub fn with_api_groups(mut self, api_groups: Vec<String>) -> RbacV1alpha1PolicyRule {
    self.api_groups = Some(api_groups);
    self
  }

  pub fn api_groups(&self) -> Option<&Vec<String>> {
    self.api_groups.as_ref()
  }

  pub fn reset_api_groups(&mut self) {
    self.api_groups = None;
  }

  pub fn set_non_resource_ur_ls(&mut self, non_resource_ur_ls: Vec<String>) {
    self.non_resource_ur_ls = Some(non_resource_ur_ls);
  }

  pub fn with_non_resource_ur_ls(mut self, non_resource_ur_ls: Vec<String>) -> RbacV1alpha1PolicyRule {
    self.non_resource_ur_ls = Some(non_resource_ur_ls);
    self
  }

  pub fn non_resource_ur_ls(&self) -> Option<&Vec<String>> {
    self.non_resource_ur_ls.as_ref()
  }

  pub fn reset_non_resource_ur_ls(&mut self) {
    self.non_resource_ur_ls = None;
  }

  pub fn set_resource_names(&mut self, resource_names: Vec<String>) {
    self.resource_names = Some(resource_names);
  }

  pub fn with_resource_names(mut self, resource_names: Vec<String>) -> RbacV1alpha1PolicyRule {
    self.resource_names = Some(resource_names);
    self
  }

  pub fn resource_names(&self) -> Option<&Vec<String>> {
    self.resource_names.as_ref()
  }

  pub fn reset_resource_names(&mut self) {
    self.resource_names = None;
  }

  pub fn set_resources(&mut self, resources: Vec<String>) {
    self.resources = Some(resources);
  }

  pub fn with_resources(mut self, resources: Vec<String>) -> RbacV1alpha1PolicyRule {
    self.resources = Some(resources);
    self
  }

  pub fn resources(&self) -> Option<&Vec<String>> {
    self.resources.as_ref()
  }

  pub fn reset_resources(&mut self) {
    self.resources = None;
  }

  pub fn set_verbs(&mut self, verbs: Vec<String>) {
    self.verbs = verbs;
  }

  pub fn with_verbs(mut self, verbs: Vec<String>) -> RbacV1alpha1PolicyRule {
    self.verbs = verbs;
    self
  }

  pub fn verbs(&self) -> &Vec<String> {
    &self.verbs
  }


}



