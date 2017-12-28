/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1PodSecurityContext : PodSecurityContext holds pod-level security attributes and common container settings. Some fields are also present in container.securityContext.  Field values of container.securityContext take precedence over field values of PodSecurityContext.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1PodSecurityContext {
  /// A special supplemental group that applies to all containers in a pod. Some volume types allow the Kubelet to change the ownership of that volume to be owned by the pod:  1. The owning GID will be the FSGroup 2. The setgid bit is set (new files created in the volume will be owned by FSGroup) 3. The permission bits are OR'd with rw-rw----  If unset, the Kubelet will not modify the ownership and permissions of any volume.
  #[serde(rename = "fsGroup")]
  fs_group: Option<i64>,
  /// Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
  #[serde(rename = "runAsNonRoot")]
  run_as_non_root: Option<bool>,
  /// The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.
  #[serde(rename = "runAsUser")]
  run_as_user: Option<i64>,
  /// The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.
  #[serde(rename = "seLinuxOptions")]
  se_linux_options: Option<::models::V1SeLinuxOptions>,
  /// A list of groups applied to the first process run in each container, in addition to the container's primary GID.  If unspecified, no groups will be added to any container.
  #[serde(rename = "supplementalGroups")]
  supplemental_groups: Option<Vec<i64>>
}

impl V1PodSecurityContext {
  /// PodSecurityContext holds pod-level security attributes and common container settings. Some fields are also present in container.securityContext.  Field values of container.securityContext take precedence over field values of PodSecurityContext.
  pub fn new() -> V1PodSecurityContext {
    V1PodSecurityContext {
      fs_group: None,
      run_as_non_root: None,
      run_as_user: None,
      se_linux_options: None,
      supplemental_groups: None
    }
  }

  pub fn set_fs_group(&mut self, fs_group: i64) {
    self.fs_group = Some(fs_group);
  }

  pub fn with_fs_group(mut self, fs_group: i64) -> V1PodSecurityContext {
    self.fs_group = Some(fs_group);
    self
  }

  pub fn fs_group(&self) -> Option<&i64> {
    self.fs_group.as_ref()
  }

  pub fn reset_fs_group(&mut self) {
    self.fs_group = None;
  }

  pub fn set_run_as_non_root(&mut self, run_as_non_root: bool) {
    self.run_as_non_root = Some(run_as_non_root);
  }

  pub fn with_run_as_non_root(mut self, run_as_non_root: bool) -> V1PodSecurityContext {
    self.run_as_non_root = Some(run_as_non_root);
    self
  }

  pub fn run_as_non_root(&self) -> Option<&bool> {
    self.run_as_non_root.as_ref()
  }

  pub fn reset_run_as_non_root(&mut self) {
    self.run_as_non_root = None;
  }

  pub fn set_run_as_user(&mut self, run_as_user: i64) {
    self.run_as_user = Some(run_as_user);
  }

  pub fn with_run_as_user(mut self, run_as_user: i64) -> V1PodSecurityContext {
    self.run_as_user = Some(run_as_user);
    self
  }

  pub fn run_as_user(&self) -> Option<&i64> {
    self.run_as_user.as_ref()
  }

  pub fn reset_run_as_user(&mut self) {
    self.run_as_user = None;
  }

  pub fn set_se_linux_options(&mut self, se_linux_options: ::models::V1SeLinuxOptions) {
    self.se_linux_options = Some(se_linux_options);
  }

  pub fn with_se_linux_options(mut self, se_linux_options: ::models::V1SeLinuxOptions) -> V1PodSecurityContext {
    self.se_linux_options = Some(se_linux_options);
    self
  }

  pub fn se_linux_options(&self) -> Option<&::models::V1SeLinuxOptions> {
    self.se_linux_options.as_ref()
  }

  pub fn reset_se_linux_options(&mut self) {
    self.se_linux_options = None;
  }

  pub fn set_supplemental_groups(&mut self, supplemental_groups: Vec<i64>) {
    self.supplemental_groups = Some(supplemental_groups);
  }

  pub fn with_supplemental_groups(mut self, supplemental_groups: Vec<i64>) -> V1PodSecurityContext {
    self.supplemental_groups = Some(supplemental_groups);
    self
  }

  pub fn supplemental_groups(&self) -> Option<&Vec<i64>> {
    self.supplemental_groups.as_ref()
  }

  pub fn reset_supplemental_groups(&mut self) {
    self.supplemental_groups = None;
  }

}



