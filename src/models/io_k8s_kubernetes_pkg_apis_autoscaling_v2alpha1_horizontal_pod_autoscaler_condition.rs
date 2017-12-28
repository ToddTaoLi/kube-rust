/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AutoscalingV2alpha1HorizontalPodAutoscalerCondition : HorizontalPodAutoscalerCondition describes the state of a HorizontalPodAutoscaler at a certain point.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AutoscalingV2alpha1HorizontalPodAutoscalerCondition {
  /// lastTransitionTime is the last time the condition transitioned from one status to another
  #[serde(rename = "lastTransitionTime")]
  last_transition_time: Option<::models::MetaV1Time>,
  /// message is a human-readable explanation containing details about the transition
  #[serde(rename = "message")]
  message: Option<String>,
  /// reason is the reason for the condition's last transition.
  #[serde(rename = "reason")]
  reason: Option<String>,
  /// status is the status of the condition (True, False, Unknown)
  #[serde(rename = "status")]
  status: String,
  /// type describes the current condition
  #[serde(rename = "type")]
  _type: String
}

impl AutoscalingV2alpha1HorizontalPodAutoscalerCondition {
  /// HorizontalPodAutoscalerCondition describes the state of a HorizontalPodAutoscaler at a certain point.
  pub fn new(status: String, _type: String) -> AutoscalingV2alpha1HorizontalPodAutoscalerCondition {
    AutoscalingV2alpha1HorizontalPodAutoscalerCondition {
      last_transition_time: None,
      message: None,
      reason: None,
      status: status,
      _type: _type
    }
  }

  pub fn set_last_transition_time(&mut self, last_transition_time: ::models::MetaV1Time) {
    self.last_transition_time = Some(last_transition_time);
  }

  pub fn with_last_transition_time(mut self, last_transition_time: ::models::MetaV1Time) -> AutoscalingV2alpha1HorizontalPodAutoscalerCondition {
    self.last_transition_time = Some(last_transition_time);
    self
  }

  pub fn last_transition_time(&self) -> Option<&::models::MetaV1Time> {
    self.last_transition_time.as_ref()
  }

  pub fn reset_last_transition_time(&mut self) {
    self.last_transition_time = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> AutoscalingV2alpha1HorizontalPodAutoscalerCondition {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_reason(&mut self, reason: String) {
    self.reason = Some(reason);
  }

  pub fn with_reason(mut self, reason: String) -> AutoscalingV2alpha1HorizontalPodAutoscalerCondition {
    self.reason = Some(reason);
    self
  }

  pub fn reason(&self) -> Option<&String> {
    self.reason.as_ref()
  }

  pub fn reset_reason(&mut self) {
    self.reason = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }

  pub fn with_status(mut self, status: String) -> AutoscalingV2alpha1HorizontalPodAutoscalerCondition {
    self.status = status;
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> AutoscalingV2alpha1HorizontalPodAutoscalerCondition {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


}



