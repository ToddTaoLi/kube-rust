/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AppsV1beta1StatefulSetStatus : StatefulSetStatus represents the current state of a StatefulSet.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppsV1beta1StatefulSetStatus {
  /// currentReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by currentRevision.
  #[serde(rename = "currentReplicas")]
  current_replicas: Option<i32>,
  /// currentRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence [0,currentReplicas).
  #[serde(rename = "currentRevision")]
  current_revision: Option<String>,
  /// observedGeneration is the most recent generation observed for this StatefulSet. It corresponds to the StatefulSet's generation, which is updated on mutation by the API Server.
  #[serde(rename = "observedGeneration")]
  observed_generation: Option<i64>,
  /// readyReplicas is the number of Pods created by the StatefulSet controller that have a Ready Condition.
  #[serde(rename = "readyReplicas")]
  ready_replicas: Option<i32>,
  /// replicas is the number of Pods created by the StatefulSet controller.
  #[serde(rename = "replicas")]
  replicas: i32,
  /// updateRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence [replicas-updatedReplicas,replicas)
  #[serde(rename = "updateRevision")]
  update_revision: Option<String>,
  /// updatedReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by updateRevision.
  #[serde(rename = "updatedReplicas")]
  updated_replicas: Option<i32>
}

impl AppsV1beta1StatefulSetStatus {
  /// StatefulSetStatus represents the current state of a StatefulSet.
  pub fn new(replicas: i32) -> AppsV1beta1StatefulSetStatus {
    AppsV1beta1StatefulSetStatus {
      current_replicas: None,
      current_revision: None,
      observed_generation: None,
      ready_replicas: None,
      replicas: replicas,
      update_revision: None,
      updated_replicas: None
    }
  }

  pub fn set_current_replicas(&mut self, current_replicas: i32) {
    self.current_replicas = Some(current_replicas);
  }

  pub fn with_current_replicas(mut self, current_replicas: i32) -> AppsV1beta1StatefulSetStatus {
    self.current_replicas = Some(current_replicas);
    self
  }

  pub fn current_replicas(&self) -> Option<&i32> {
    self.current_replicas.as_ref()
  }

  pub fn reset_current_replicas(&mut self) {
    self.current_replicas = None;
  }

  pub fn set_current_revision(&mut self, current_revision: String) {
    self.current_revision = Some(current_revision);
  }

  pub fn with_current_revision(mut self, current_revision: String) -> AppsV1beta1StatefulSetStatus {
    self.current_revision = Some(current_revision);
    self
  }

  pub fn current_revision(&self) -> Option<&String> {
    self.current_revision.as_ref()
  }

  pub fn reset_current_revision(&mut self) {
    self.current_revision = None;
  }

  pub fn set_observed_generation(&mut self, observed_generation: i64) {
    self.observed_generation = Some(observed_generation);
  }

  pub fn with_observed_generation(mut self, observed_generation: i64) -> AppsV1beta1StatefulSetStatus {
    self.observed_generation = Some(observed_generation);
    self
  }

  pub fn observed_generation(&self) -> Option<&i64> {
    self.observed_generation.as_ref()
  }

  pub fn reset_observed_generation(&mut self) {
    self.observed_generation = None;
  }

  pub fn set_ready_replicas(&mut self, ready_replicas: i32) {
    self.ready_replicas = Some(ready_replicas);
  }

  pub fn with_ready_replicas(mut self, ready_replicas: i32) -> AppsV1beta1StatefulSetStatus {
    self.ready_replicas = Some(ready_replicas);
    self
  }

  pub fn ready_replicas(&self) -> Option<&i32> {
    self.ready_replicas.as_ref()
  }

  pub fn reset_ready_replicas(&mut self) {
    self.ready_replicas = None;
  }

  pub fn set_replicas(&mut self, replicas: i32) {
    self.replicas = replicas;
  }

  pub fn with_replicas(mut self, replicas: i32) -> AppsV1beta1StatefulSetStatus {
    self.replicas = replicas;
    self
  }

  pub fn replicas(&self) -> &i32 {
    &self.replicas
  }


  pub fn set_update_revision(&mut self, update_revision: String) {
    self.update_revision = Some(update_revision);
  }

  pub fn with_update_revision(mut self, update_revision: String) -> AppsV1beta1StatefulSetStatus {
    self.update_revision = Some(update_revision);
    self
  }

  pub fn update_revision(&self) -> Option<&String> {
    self.update_revision.as_ref()
  }

  pub fn reset_update_revision(&mut self) {
    self.update_revision = None;
  }

  pub fn set_updated_replicas(&mut self, updated_replicas: i32) {
    self.updated_replicas = Some(updated_replicas);
  }

  pub fn with_updated_replicas(mut self, updated_replicas: i32) -> AppsV1beta1StatefulSetStatus {
    self.updated_replicas = Some(updated_replicas);
    self
  }

  pub fn updated_replicas(&self) -> Option<&i32> {
    self.updated_replicas.as_ref()
  }

  pub fn reset_updated_replicas(&mut self) {
    self.updated_replicas = None;
  }

}



