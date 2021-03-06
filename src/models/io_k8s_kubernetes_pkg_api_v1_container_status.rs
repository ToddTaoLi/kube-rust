/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1ContainerStatus : ContainerStatus contains details for the current status of this container.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1ContainerStatus {
  /// Container's ID in the format 'docker://<container_id>'.
  #[serde(rename = "containerID")]
  container_id: Option<String>,
  /// The image the container is running. More info: https://kubernetes.io/docs/concepts/containers/images
  #[serde(rename = "image")]
  image: String,
  /// ImageID of the container's image.
  #[serde(rename = "imageID")]
  image_id: String,
  /// Details about the container's last termination condition.
  #[serde(rename = "lastState")]
  last_state: Option<::models::V1ContainerState>,
  /// This must be a DNS_LABEL. Each container in a pod must have a unique name. Cannot be updated.
  #[serde(rename = "name")]
  name: String,
  /// Specifies whether the container has passed its readiness probe.
  #[serde(rename = "ready")]
  ready: bool,
  /// The number of times the container has been restarted, currently based on the number of dead containers that have not yet been removed. Note that this is calculated from dead containers. But those containers are subject to garbage collection. This value will get capped at 5 by GC.
  #[serde(rename = "restartCount")]
  restart_count: i32,
  /// Details about the container's current condition.
  #[serde(rename = "state")]
  state: Option<::models::V1ContainerState>
}

impl V1ContainerStatus {
  /// ContainerStatus contains details for the current status of this container.
  pub fn new(image: String, image_id: String, name: String, ready: bool, restart_count: i32) -> V1ContainerStatus {
    V1ContainerStatus {
      container_id: None,
      image: image,
      image_id: image_id,
      last_state: None,
      name: name,
      ready: ready,
      restart_count: restart_count,
      state: None
    }
  }

  pub fn set_container_id(&mut self, container_id: String) {
    self.container_id = Some(container_id);
  }

  pub fn with_container_id(mut self, container_id: String) -> V1ContainerStatus {
    self.container_id = Some(container_id);
    self
  }

  pub fn container_id(&self) -> Option<&String> {
    self.container_id.as_ref()
  }

  pub fn reset_container_id(&mut self) {
    self.container_id = None;
  }

  pub fn set_image(&mut self, image: String) {
    self.image = image;
  }

  pub fn with_image(mut self, image: String) -> V1ContainerStatus {
    self.image = image;
    self
  }

  pub fn image(&self) -> &String {
    &self.image
  }


  pub fn set_image_id(&mut self, image_id: String) {
    self.image_id = image_id;
  }

  pub fn with_image_id(mut self, image_id: String) -> V1ContainerStatus {
    self.image_id = image_id;
    self
  }

  pub fn image_id(&self) -> &String {
    &self.image_id
  }


  pub fn set_last_state(&mut self, last_state: ::models::V1ContainerState) {
    self.last_state = Some(last_state);
  }

  pub fn with_last_state(mut self, last_state: ::models::V1ContainerState) -> V1ContainerStatus {
    self.last_state = Some(last_state);
    self
  }

  pub fn last_state(&self) -> Option<&::models::V1ContainerState> {
    self.last_state.as_ref()
  }

  pub fn reset_last_state(&mut self) {
    self.last_state = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> V1ContainerStatus {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_ready(&mut self, ready: bool) {
    self.ready = ready;
  }

  pub fn with_ready(mut self, ready: bool) -> V1ContainerStatus {
    self.ready = ready;
    self
  }

  pub fn ready(&self) -> &bool {
    &self.ready
  }


  pub fn set_restart_count(&mut self, restart_count: i32) {
    self.restart_count = restart_count;
  }

  pub fn with_restart_count(mut self, restart_count: i32) -> V1ContainerStatus {
    self.restart_count = restart_count;
    self
  }

  pub fn restart_count(&self) -> &i32 {
    &self.restart_count
  }


  pub fn set_state(&mut self, state: ::models::V1ContainerState) {
    self.state = Some(state);
  }

  pub fn with_state(mut self, state: ::models::V1ContainerState) -> V1ContainerStatus {
    self.state = Some(state);
    self
  }

  pub fn state(&self) -> Option<&::models::V1ContainerState> {
    self.state.as_ref()
  }

  pub fn reset_state(&mut self) {
    self.state = None;
  }

}



