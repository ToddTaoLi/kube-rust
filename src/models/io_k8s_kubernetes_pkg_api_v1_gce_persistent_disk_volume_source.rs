/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1GcePersistentDiskVolumeSource : Represents a Persistent Disk resource in Google Compute Engine.  A GCE PD must exist before mounting to a container. The disk must also be in the same GCE project and zone as the kubelet. A GCE PD can only be mounted as read/write once or read-only many times. GCE PDs support ownership management and SELinux relabeling.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1GcePersistentDiskVolumeSource {
  /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
  #[serde(rename = "fsType")]
  fs_type: Option<String>,
  /// The partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as \"1\". Similarly, the volume partition for /dev/sda is \"0\" (or you can leave the property empty). More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
  #[serde(rename = "partition")]
  partition: Option<i32>,
  /// Unique name of the PD resource in GCE. Used to identify the disk in GCE. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
  #[serde(rename = "pdName")]
  pd_name: String,
  /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
  #[serde(rename = "readOnly")]
  read_only: Option<bool>
}

impl V1GcePersistentDiskVolumeSource {
  /// Represents a Persistent Disk resource in Google Compute Engine.  A GCE PD must exist before mounting to a container. The disk must also be in the same GCE project and zone as the kubelet. A GCE PD can only be mounted as read/write once or read-only many times. GCE PDs support ownership management and SELinux relabeling.
  pub fn new(pd_name: String) -> V1GcePersistentDiskVolumeSource {
    V1GcePersistentDiskVolumeSource {
      fs_type: None,
      partition: None,
      pd_name: pd_name,
      read_only: None
    }
  }

  pub fn set_fs_type(&mut self, fs_type: String) {
    self.fs_type = Some(fs_type);
  }

  pub fn with_fs_type(mut self, fs_type: String) -> V1GcePersistentDiskVolumeSource {
    self.fs_type = Some(fs_type);
    self
  }

  pub fn fs_type(&self) -> Option<&String> {
    self.fs_type.as_ref()
  }

  pub fn reset_fs_type(&mut self) {
    self.fs_type = None;
  }

  pub fn set_partition(&mut self, partition: i32) {
    self.partition = Some(partition);
  }

  pub fn with_partition(mut self, partition: i32) -> V1GcePersistentDiskVolumeSource {
    self.partition = Some(partition);
    self
  }

  pub fn partition(&self) -> Option<&i32> {
    self.partition.as_ref()
  }

  pub fn reset_partition(&mut self) {
    self.partition = None;
  }

  pub fn set_pd_name(&mut self, pd_name: String) {
    self.pd_name = pd_name;
  }

  pub fn with_pd_name(mut self, pd_name: String) -> V1GcePersistentDiskVolumeSource {
    self.pd_name = pd_name;
    self
  }

  pub fn pd_name(&self) -> &String {
    &self.pd_name
  }


  pub fn set_read_only(&mut self, read_only: bool) {
    self.read_only = Some(read_only);
  }

  pub fn with_read_only(mut self, read_only: bool) -> V1GcePersistentDiskVolumeSource {
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



