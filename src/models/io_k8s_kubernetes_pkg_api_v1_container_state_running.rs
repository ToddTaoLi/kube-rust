/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1ContainerStateRunning : ContainerStateRunning is a running state of a container.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1ContainerStateRunning {
  /// Time at which the container was last (re-)started
  #[serde(rename = "startedAt")]
  started_at: Option<::models::MetaV1Time>
}

impl V1ContainerStateRunning {
  /// ContainerStateRunning is a running state of a container.
  pub fn new() -> V1ContainerStateRunning {
    V1ContainerStateRunning {
      started_at: None
    }
  }

  pub fn set_started_at(&mut self, started_at: ::models::MetaV1Time) {
    self.started_at = Some(started_at);
  }

  pub fn with_started_at(mut self, started_at: ::models::MetaV1Time) -> V1ContainerStateRunning {
    self.started_at = Some(started_at);
    self
  }

  pub fn started_at(&self) -> Option<&::models::MetaV1Time> {
    self.started_at.as_ref()
  }

  pub fn reset_started_at(&mut self) {
    self.started_at = None;
  }

}



