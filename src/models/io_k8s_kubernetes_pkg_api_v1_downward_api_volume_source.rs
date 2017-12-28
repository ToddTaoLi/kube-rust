/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1DownwardApiVolumeSource : DownwardAPIVolumeSource represents a volume containing downward API info. Downward API volumes support ownership management and SELinux relabeling.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1DownwardApiVolumeSource {
  /// Optional: mode bits to use on created files by default. Must be a value between 0 and 0777. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
  #[serde(rename = "defaultMode")]
  default_mode: Option<i32>,
  /// Items is a list of downward API volume file
  #[serde(rename = "items")]
  items: Option<Vec<::models::V1DownwardApiVolumeFile>>
}

impl V1DownwardApiVolumeSource {
  /// DownwardAPIVolumeSource represents a volume containing downward API info. Downward API volumes support ownership management and SELinux relabeling.
  pub fn new() -> V1DownwardApiVolumeSource {
    V1DownwardApiVolumeSource {
      default_mode: None,
      items: None
    }
  }

  pub fn set_default_mode(&mut self, default_mode: i32) {
    self.default_mode = Some(default_mode);
  }

  pub fn with_default_mode(mut self, default_mode: i32) -> V1DownwardApiVolumeSource {
    self.default_mode = Some(default_mode);
    self
  }

  pub fn default_mode(&self) -> Option<&i32> {
    self.default_mode.as_ref()
  }

  pub fn reset_default_mode(&mut self) {
    self.default_mode = None;
  }

  pub fn set_items(&mut self, items: Vec<::models::V1DownwardApiVolumeFile>) {
    self.items = Some(items);
  }

  pub fn with_items(mut self, items: Vec<::models::V1DownwardApiVolumeFile>) -> V1DownwardApiVolumeSource {
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



