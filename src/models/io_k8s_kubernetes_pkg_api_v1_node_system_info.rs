/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1NodeSystemInfo : NodeSystemInfo is a set of ids/uuids to uniquely identify the node.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1NodeSystemInfo {
  /// The Architecture reported by the node
  #[serde(rename = "architecture")]
  architecture: String,
  /// Boot ID reported by the node.
  #[serde(rename = "bootID")]
  boot_id: String,
  /// ContainerRuntime Version reported by the node through runtime remote API (e.g. docker://1.5.0).
  #[serde(rename = "containerRuntimeVersion")]
  container_runtime_version: String,
  /// Kernel Version reported by the node from 'uname -r' (e.g. 3.16.0-0.bpo.4-amd64).
  #[serde(rename = "kernelVersion")]
  kernel_version: String,
  /// KubeProxy Version reported by the node.
  #[serde(rename = "kubeProxyVersion")]
  kube_proxy_version: String,
  /// Kubelet Version reported by the node.
  #[serde(rename = "kubeletVersion")]
  kubelet_version: String,
  /// MachineID reported by the node. For unique machine identification in the cluster this field is preferred. Learn more from man(5) machine-id: http://man7.org/linux/man-pages/man5/machine-id.5.html
  #[serde(rename = "machineID")]
  machine_id: String,
  /// The Operating System reported by the node
  #[serde(rename = "operatingSystem")]
  operating_system: String,
  /// OS Image reported by the node from /etc/os-release (e.g. Debian GNU/Linux 7 (wheezy)).
  #[serde(rename = "osImage")]
  os_image: String,
  /// SystemUUID reported by the node. For unique machine identification MachineID is preferred. This field is specific to Red Hat hosts https://access.redhat.com/documentation/en-US/Red_Hat_Subscription_Management/1/html/RHSM/getting-system-uuid.html
  #[serde(rename = "systemUUID")]
  system_uuid: String
}

impl V1NodeSystemInfo {
  /// NodeSystemInfo is a set of ids/uuids to uniquely identify the node.
  pub fn new(architecture: String, boot_id: String, container_runtime_version: String, kernel_version: String, kube_proxy_version: String, kubelet_version: String, machine_id: String, operating_system: String, os_image: String, system_uuid: String) -> V1NodeSystemInfo {
    V1NodeSystemInfo {
      architecture: architecture,
      boot_id: boot_id,
      container_runtime_version: container_runtime_version,
      kernel_version: kernel_version,
      kube_proxy_version: kube_proxy_version,
      kubelet_version: kubelet_version,
      machine_id: machine_id,
      operating_system: operating_system,
      os_image: os_image,
      system_uuid: system_uuid
    }
  }

  pub fn set_architecture(&mut self, architecture: String) {
    self.architecture = architecture;
  }

  pub fn with_architecture(mut self, architecture: String) -> V1NodeSystemInfo {
    self.architecture = architecture;
    self
  }

  pub fn architecture(&self) -> &String {
    &self.architecture
  }


  pub fn set_boot_id(&mut self, boot_id: String) {
    self.boot_id = boot_id;
  }

  pub fn with_boot_id(mut self, boot_id: String) -> V1NodeSystemInfo {
    self.boot_id = boot_id;
    self
  }

  pub fn boot_id(&self) -> &String {
    &self.boot_id
  }


  pub fn set_container_runtime_version(&mut self, container_runtime_version: String) {
    self.container_runtime_version = container_runtime_version;
  }

  pub fn with_container_runtime_version(mut self, container_runtime_version: String) -> V1NodeSystemInfo {
    self.container_runtime_version = container_runtime_version;
    self
  }

  pub fn container_runtime_version(&self) -> &String {
    &self.container_runtime_version
  }


  pub fn set_kernel_version(&mut self, kernel_version: String) {
    self.kernel_version = kernel_version;
  }

  pub fn with_kernel_version(mut self, kernel_version: String) -> V1NodeSystemInfo {
    self.kernel_version = kernel_version;
    self
  }

  pub fn kernel_version(&self) -> &String {
    &self.kernel_version
  }


  pub fn set_kube_proxy_version(&mut self, kube_proxy_version: String) {
    self.kube_proxy_version = kube_proxy_version;
  }

  pub fn with_kube_proxy_version(mut self, kube_proxy_version: String) -> V1NodeSystemInfo {
    self.kube_proxy_version = kube_proxy_version;
    self
  }

  pub fn kube_proxy_version(&self) -> &String {
    &self.kube_proxy_version
  }


  pub fn set_kubelet_version(&mut self, kubelet_version: String) {
    self.kubelet_version = kubelet_version;
  }

  pub fn with_kubelet_version(mut self, kubelet_version: String) -> V1NodeSystemInfo {
    self.kubelet_version = kubelet_version;
    self
  }

  pub fn kubelet_version(&self) -> &String {
    &self.kubelet_version
  }


  pub fn set_machine_id(&mut self, machine_id: String) {
    self.machine_id = machine_id;
  }

  pub fn with_machine_id(mut self, machine_id: String) -> V1NodeSystemInfo {
    self.machine_id = machine_id;
    self
  }

  pub fn machine_id(&self) -> &String {
    &self.machine_id
  }


  pub fn set_operating_system(&mut self, operating_system: String) {
    self.operating_system = operating_system;
  }

  pub fn with_operating_system(mut self, operating_system: String) -> V1NodeSystemInfo {
    self.operating_system = operating_system;
    self
  }

  pub fn operating_system(&self) -> &String {
    &self.operating_system
  }


  pub fn set_os_image(&mut self, os_image: String) {
    self.os_image = os_image;
  }

  pub fn with_os_image(mut self, os_image: String) -> V1NodeSystemInfo {
    self.os_image = os_image;
    self
  }

  pub fn os_image(&self) -> &String {
    &self.os_image
  }


  pub fn set_system_uuid(&mut self, system_uuid: String) {
    self.system_uuid = system_uuid;
  }

  pub fn with_system_uuid(mut self, system_uuid: String) -> V1NodeSystemInfo {
    self.system_uuid = system_uuid;
    self
  }

  pub fn system_uuid(&self) -> &String {
    &self.system_uuid
  }


}



