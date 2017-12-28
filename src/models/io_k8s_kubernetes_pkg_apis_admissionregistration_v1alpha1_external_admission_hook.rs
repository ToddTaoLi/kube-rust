/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AdmissionregistrationV1alpha1ExternalAdmissionHook : ExternalAdmissionHook describes an external admission webhook and the resources and operations it applies to.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdmissionregistrationV1alpha1ExternalAdmissionHook {
  /// ClientConfig defines how to communicate with the hook. Required
  #[serde(rename = "clientConfig")]
  client_config: ::models::AdmissionregistrationV1alpha1AdmissionHookClientConfig,
  /// FailurePolicy defines how unrecognized errors from the admission endpoint are handled - allowed values are Ignore or Fail. Defaults to Ignore.
  #[serde(rename = "failurePolicy")]
  failure_policy: Option<String>,
  /// The name of the external admission webhook. Name should be fully qualified, e.g., imagepolicy.kubernetes.io, where \"imagepolicy\" is the name of the webhook, and kubernetes.io is the name of the organization. Required.
  #[serde(rename = "name")]
  name: String,
  /// Rules describes what operations on what resources/subresources the webhook cares about. The webhook cares about an operation if it matches _any_ Rule.
  #[serde(rename = "rules")]
  rules: Option<Vec<::models::AdmissionregistrationV1alpha1RuleWithOperations>>
}

impl AdmissionregistrationV1alpha1ExternalAdmissionHook {
  /// ExternalAdmissionHook describes an external admission webhook and the resources and operations it applies to.
  pub fn new(client_config: ::models::AdmissionregistrationV1alpha1AdmissionHookClientConfig, name: String) -> AdmissionregistrationV1alpha1ExternalAdmissionHook {
    AdmissionregistrationV1alpha1ExternalAdmissionHook {
      client_config: client_config,
      failure_policy: None,
      name: name,
      rules: None
    }
  }

  pub fn set_client_config(&mut self, client_config: ::models::AdmissionregistrationV1alpha1AdmissionHookClientConfig) {
    self.client_config = client_config;
  }

  pub fn with_client_config(mut self, client_config: ::models::AdmissionregistrationV1alpha1AdmissionHookClientConfig) -> AdmissionregistrationV1alpha1ExternalAdmissionHook {
    self.client_config = client_config;
    self
  }

  pub fn client_config(&self) -> &::models::AdmissionregistrationV1alpha1AdmissionHookClientConfig {
    &self.client_config
  }


  pub fn set_failure_policy(&mut self, failure_policy: String) {
    self.failure_policy = Some(failure_policy);
  }

  pub fn with_failure_policy(mut self, failure_policy: String) -> AdmissionregistrationV1alpha1ExternalAdmissionHook {
    self.failure_policy = Some(failure_policy);
    self
  }

  pub fn failure_policy(&self) -> Option<&String> {
    self.failure_policy.as_ref()
  }

  pub fn reset_failure_policy(&mut self) {
    self.failure_policy = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> AdmissionregistrationV1alpha1ExternalAdmissionHook {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_rules(&mut self, rules: Vec<::models::AdmissionregistrationV1alpha1RuleWithOperations>) {
    self.rules = Some(rules);
  }

  pub fn with_rules(mut self, rules: Vec<::models::AdmissionregistrationV1alpha1RuleWithOperations>) -> AdmissionregistrationV1alpha1ExternalAdmissionHook {
    self.rules = Some(rules);
    self
  }

  pub fn rules(&self) -> Option<&Vec<::models::AdmissionregistrationV1alpha1RuleWithOperations>> {
    self.rules.as_ref()
  }

  pub fn reset_rules(&mut self) {
    self.rules = None;
  }

}



