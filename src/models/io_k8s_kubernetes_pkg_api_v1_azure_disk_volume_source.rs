/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1AzureDiskVolumeSource : AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1AzureDiskVolumeSource {
  /// Host Caching mode: None, Read Only, Read Write.
  #[serde(rename = "cachingMode")]
  caching_mode: Option<String>,
  /// The Name of the data disk in the blob storage
  #[serde(rename = "diskName")]
  disk_name: String,
  /// The URI the data disk in the blob storage
  #[serde(rename = "diskURI")]
  disk_uri: String,
  /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified.
  #[serde(rename = "fsType")]
  fs_type: Option<String>,
  /// Expected values Shared: mulitple blob disks per storage account  Dedicated: single blob disk per storage account  Managed: azure managed data disk (only in managed availability set). defaults to shared
  #[serde(rename = "kind")]
  kind: Option<String>,
  /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
  #[serde(rename = "readOnly")]
  read_only: Option<bool>
}

impl V1AzureDiskVolumeSource {
  /// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
  pub fn new(disk_name: String, disk_uri: String) -> V1AzureDiskVolumeSource {
    V1AzureDiskVolumeSource {
      caching_mode: None,
      disk_name: disk_name,
      disk_uri: disk_uri,
      fs_type: None,
      kind: None,
      read_only: None
    }
  }

  pub fn set_caching_mode(&mut self, caching_mode: String) {
    self.caching_mode = Some(caching_mode);
  }

  pub fn with_caching_mode(mut self, caching_mode: String) -> V1AzureDiskVolumeSource {
    self.caching_mode = Some(caching_mode);
    self
  }

  pub fn caching_mode(&self) -> Option<&String> {
    self.caching_mode.as_ref()
  }

  pub fn reset_caching_mode(&mut self) {
    self.caching_mode = None;
  }

  pub fn set_disk_name(&mut self, disk_name: String) {
    self.disk_name = disk_name;
  }

  pub fn with_disk_name(mut self, disk_name: String) -> V1AzureDiskVolumeSource {
    self.disk_name = disk_name;
    self
  }

  pub fn disk_name(&self) -> &String {
    &self.disk_name
  }


  pub fn set_disk_uri(&mut self, disk_uri: String) {
    self.disk_uri = disk_uri;
  }

  pub fn with_disk_uri(mut self, disk_uri: String) -> V1AzureDiskVolumeSource {
    self.disk_uri = disk_uri;
    self
  }

  pub fn disk_uri(&self) -> &String {
    &self.disk_uri
  }


  pub fn set_fs_type(&mut self, fs_type: String) {
    self.fs_type = Some(fs_type);
  }

  pub fn with_fs_type(mut self, fs_type: String) -> V1AzureDiskVolumeSource {
    self.fs_type = Some(fs_type);
    self
  }

  pub fn fs_type(&self) -> Option<&String> {
    self.fs_type.as_ref()
  }

  pub fn reset_fs_type(&mut self) {
    self.fs_type = None;
  }

  pub fn set_kind(&mut self, kind: String) {
    self.kind = Some(kind);
  }

  pub fn with_kind(mut self, kind: String) -> V1AzureDiskVolumeSource {
    self.kind = Some(kind);
    self
  }

  pub fn kind(&self) -> Option<&String> {
    self.kind.as_ref()
  }

  pub fn reset_kind(&mut self) {
    self.kind = None;
  }

  pub fn set_read_only(&mut self, read_only: bool) {
    self.read_only = Some(read_only);
  }

  pub fn with_read_only(mut self, read_only: bool) -> V1AzureDiskVolumeSource {
    self.read_only = Some(read_only);
    self
  }

  pub fn read_only(&self) -> Option<&bool> {
    self.read_only.as_ref()
  }

  pub fn reset_read_only(&mut self) {
    self.read_only = None;
  }

}



