/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// BatchV1JobStatus : JobStatus represents the current state of a Job.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchV1JobStatus {
  /// The number of actively running pods.
  #[serde(rename = "active")]
  active: Option<i32>,
  /// Represents time when the job was completed. It is not guaranteed to be set in happens-before order across separate operations. It is represented in RFC3339 form and is in UTC.
  #[serde(rename = "completionTime")]
  completion_time: Option<::models::MetaV1Time>,
  /// The latest available observations of an object's current state. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
  #[serde(rename = "conditions")]
  conditions: Option<Vec<::models::BatchV1JobCondition>>,
  /// The number of pods which reached phase Failed.
  #[serde(rename = "failed")]
  failed: Option<i32>,
  /// Represents time when the job was acknowledged by the job controller. It is not guaranteed to be set in happens-before order across separate operations. It is represented in RFC3339 form and is in UTC.
  #[serde(rename = "startTime")]
  start_time: Option<::models::MetaV1Time>,
  /// The number of pods which reached phase Succeeded.
  #[serde(rename = "succeeded")]
  succeeded: Option<i32>
}

impl BatchV1JobStatus {
  /// JobStatus represents the current state of a Job.
  pub fn new() -> BatchV1JobStatus {
    BatchV1JobStatus {
      active: None,
      completion_time: None,
      conditions: None,
      failed: None,
      start_time: None,
      succeeded: None
    }
  }

  pub fn set_active(&mut self, active: i32) {
    self.active = Some(active);
  }

  pub fn with_active(mut self, active: i32) -> BatchV1JobStatus {
    self.active = Some(active);
    self
  }

  pub fn active(&self) -> Option<&i32> {
    self.active.as_ref()
  }

  pub fn reset_active(&mut self) {
    self.active = None;
  }

  pub fn set_completion_time(&mut self, completion_time: ::models::MetaV1Time) {
    self.completion_time = Some(completion_time);
  }

  pub fn with_completion_time(mut self, completion_time: ::models::MetaV1Time) -> BatchV1JobStatus {
    self.completion_time = Some(completion_time);
    self
  }

  pub fn completion_time(&self) -> Option<&::models::MetaV1Time> {
    self.completion_time.as_ref()
  }

  pub fn reset_completion_time(&mut self) {
    self.completion_time = None;
  }

  pub fn set_conditions(&mut self, conditions: Vec<::models::BatchV1JobCondition>) {
    self.conditions = Some(conditions);
  }

  pub fn with_conditions(mut self, conditions: Vec<::models::BatchV1JobCondition>) -> BatchV1JobStatus {
    self.conditions = Some(conditions);
    self
  }

  pub fn conditions(&self) -> Option<&Vec<::models::BatchV1JobCondition>> {
    self.conditions.as_ref()
  }

  pub fn reset_conditions(&mut self) {
    self.conditions = None;
  }

  pub fn set_failed(&mut self, failed: i32) {
    self.failed = Some(failed);
  }

  pub fn with_failed(mut self, failed: i32) -> BatchV1JobStatus {
    self.failed = Some(failed);
    self
  }

  pub fn failed(&self) -> Option<&i32> {
    self.failed.as_ref()
  }

  pub fn reset_failed(&mut self) {
    self.failed = None;
  }

  pub fn set_start_time(&mut self, start_time: ::models::MetaV1Time) {
    self.start_time = Some(start_time);
  }

  pub fn with_start_time(mut self, start_time: ::models::MetaV1Time) -> BatchV1JobStatus {
    self.start_time = Some(start_time);
    self
  }

  pub fn start_time(&self) -> Option<&::models::MetaV1Time> {
    self.start_time.as_ref()
  }

  pub fn reset_start_time(&mut self) {
    self.start_time = None;
  }

  pub fn set_succeeded(&mut self, succeeded: i32) {
    self.succeeded = Some(succeeded);
  }

  pub fn with_succeeded(mut self, succeeded: i32) -> BatchV1JobStatus {
    self.succeeded = Some(succeeded);
    self
  }

  pub fn succeeded(&self) -> Option<&i32> {
    self.succeeded.as_ref()
  }

  pub fn reset_succeeded(&mut self) {
    self.succeeded = None;
  }

}



