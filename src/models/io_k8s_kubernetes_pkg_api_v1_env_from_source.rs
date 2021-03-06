/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1EnvFromSource : EnvFromSource represents the source of a set of ConfigMaps

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1EnvFromSource {
  /// The ConfigMap to select from
  #[serde(rename = "configMapRef")]
  config_map_ref: Option<::models::V1ConfigMapEnvSource>,
  /// An optional identifer to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER.
  #[serde(rename = "prefix")]
  prefix: Option<String>,
  /// The Secret to select from
  #[serde(rename = "secretRef")]
  secret_ref: Option<::models::V1SecretEnvSource>
}

impl V1EnvFromSource {
  /// EnvFromSource represents the source of a set of ConfigMaps
  pub fn new() -> V1EnvFromSource {
    V1EnvFromSource {
      config_map_ref: None,
      prefix: None,
      secret_ref: None
    }
  }

  pub fn set_config_map_ref(&mut self, config_map_ref: ::models::V1ConfigMapEnvSource) {
    self.config_map_ref = Some(config_map_ref);
  }

  pub fn with_config_map_ref(mut self, config_map_ref: ::models::V1ConfigMapEnvSource) -> V1EnvFromSource {
    self.config_map_ref = Some(config_map_ref);
    self
  }

  pub fn config_map_ref(&self) -> Option<&::models::V1ConfigMapEnvSource> {
    self.config_map_ref.as_ref()
  }

  pub fn reset_config_map_ref(&mut self) {
    self.config_map_ref = None;
  }

  pub fn set_prefix(&mut self, prefix: String) {
    self.prefix = Some(prefix);
  }

  pub fn with_prefix(mut self, prefix: String) -> V1EnvFromSource {
    self.prefix = Some(prefix);
    self
  }

  pub fn prefix(&self) -> Option<&String> {
    self.prefix.as_ref()
  }

  pub fn reset_prefix(&mut self) {
    self.prefix = None;
  }

  pub fn set_secret_ref(&mut self, secret_ref: ::models::V1SecretEnvSource) {
    self.secret_ref = Some(secret_ref);
  }

  pub fn with_secret_ref(mut self, secret_ref: ::models::V1SecretEnvSource) -> V1EnvFromSource {
    self.secret_ref = Some(secret_ref);
    self
  }

  pub fn secret_ref(&self) -> Option<&::models::V1SecretEnvSource> {
    self.secret_ref.as_ref()
  }

  pub fn reset_secret_ref(&mut self) {
    self.secret_ref = None;
  }

}



