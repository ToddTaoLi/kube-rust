/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1DownwardApiProjection : Represents downward API info for projecting into a projected volume. Note that this is identical to a downwardAPI volume source without the default mode.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1DownwardApiProjection {
  /// Items is a list of DownwardAPIVolume file
  #[serde(rename = "items")]
  items: Option<Vec<::models::V1DownwardApiVolumeFile>>
}

impl V1DownwardApiProjection {
  /// Represents downward API info for projecting into a projected volume. Note that this is identical to a downwardAPI volume source without the default mode.
  pub fn new() -> V1DownwardApiProjection {
    V1DownwardApiProjection {
      items: None
    }
  }

  pub fn set_items(&mut self, items: Vec<::models::V1DownwardApiVolumeFile>) {
    self.items = Some(items);
  }

  pub fn with_items(mut self, items: Vec<::models::V1DownwardApiVolumeFile>) -> V1DownwardApiProjection {
    self.items = Some(items);
    self
  }

  pub fn items(&self) -> Option<&Vec<::models::V1DownwardApiVolumeFile>> {
    self.items.as_ref()
  }

  pub fn reset_items(&mut self) {
    self.items = None;
  }

}



