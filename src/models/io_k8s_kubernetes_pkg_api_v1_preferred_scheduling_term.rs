/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1PreferredSchedulingTerm : An empty preferred scheduling term matches all objects with implicit weight 0 (i.e. it's a no-op). A null preferred scheduling term matches no objects (i.e. is also a no-op).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1PreferredSchedulingTerm {
  /// A node selector term, associated with the corresponding weight.
  #[serde(rename = "preference")]
  preference: ::models::V1NodeSelectorTerm,
  /// Weight associated with matching the corresponding nodeSelectorTerm, in the range 1-100.
  #[serde(rename = "weight")]
  weight: i32
}

impl V1PreferredSchedulingTerm {
  /// An empty preferred scheduling term matches all objects with implicit weight 0 (i.e. it's a no-op). A null preferred scheduling term matches no objects (i.e. is also a no-op).
  pub fn new(preference: ::models::V1NodeSelectorTerm, weight: i32) -> V1PreferredSchedulingTerm {
    V1PreferredSchedulingTerm {
      preference: preference,
      weight: weight
    }
  }

  pub fn set_preference(&mut self, preference: ::models::V1NodeSelectorTerm) {
    self.preference = preference;
  }

  pub fn with_preference(mut self, preference: ::models::V1NodeSelectorTerm) -> V1PreferredSchedulingTerm {
    self.preference = preference;
    self
  }

  pub fn preference(&self) -> &::models::V1NodeSelectorTerm {
    &self.preference
  }


  pub fn set_weight(&mut self, weight: i32) {
    self.weight = weight;
  }

  pub fn with_weight(mut self, weight: i32) -> V1PreferredSchedulingTerm {
    self.weight = weight;
    self
  }

  pub fn weight(&self) -> &i32 {
    &self.weight
  }


}


