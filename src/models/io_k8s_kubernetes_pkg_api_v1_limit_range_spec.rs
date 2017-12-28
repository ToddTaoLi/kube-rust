/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1LimitRangeSpec : LimitRangeSpec defines a min/max usage limit for resources that match on kind.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1LimitRangeSpec {
  /// Limits is the list of LimitRangeItem objects that are enforced.
  #[serde(rename = "limits")]
  limits: Vec<::models::V1LimitRangeItem>
}

impl V1LimitRangeSpec {
  /// LimitRangeSpec defines a min/max usage limit for resources that match on kind.
  pub fn new(limits: Vec<::models::V1LimitRangeItem>) -> V1LimitRangeSpec {
    V1LimitRangeSpec {
      limits: limits
    }
  }

  pub fn set_limits(&mut self, limits: Vec<::models::V1LimitRangeItem>) {
    self.limits = limits;
  }

  pub fn with_limits(mut self, limits: Vec<::models::V1LimitRangeItem>) -> V1LimitRangeSpec {
    self.limits = limits;
    self
  }

  pub fn limits(&self) -> &Vec<::models::V1LimitRangeItem> {
    &self.limits
  }


}



