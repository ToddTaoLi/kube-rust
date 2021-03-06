/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AppsV1beta1StatefulSetSpec : A StatefulSetSpec is the specification of a StatefulSet.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppsV1beta1StatefulSetSpec {
  /// podManagementPolicy controls how pods are created during initial scale up, when replacing pods on nodes, or when scaling down. The default policy is `OrderedReady`, where pods are created in increasing order (pod-0, then pod-1, etc) and the controller will wait until each pod is ready before continuing. When scaling down, the pods are removed in the opposite order. The alternative policy is `Parallel` which will create pods in parallel to match the desired scale without waiting, and on scale down will delete all pods at once.
  #[serde(rename = "podManagementPolicy")]
  pod_management_policy: Option<String>,
  /// replicas is the desired number of replicas of the given Template. These are replicas in the sense that they are instantiations of the same Template, but individual replicas also have a consistent identity. If unspecified, defaults to 1.
  #[serde(rename = "replicas")]
  replicas: Option<i32>,
  /// revisionHistoryLimit is the maximum number of revisions that will be maintained in the StatefulSet's revision history. The revision history consists of all revisions not represented by a currently applied StatefulSetSpec version. The default value is 10.
  #[serde(rename = "revisionHistoryLimit")]
  revision_history_limit: Option<i32>,
  /// selector is a label query over pods that should match the replica count. If empty, defaulted to labels on the pod template. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
  #[serde(rename = "selector")]
  selector: Option<::models::MetaV1LabelSelector>,
  /// serviceName is the name of the service that governs this StatefulSet. This service must exist before the StatefulSet, and is responsible for the network identity of the set. Pods get DNS/hostnames that follow the pattern: pod-specific-string.serviceName.default.svc.cluster.local where \"pod-specific-string\" is managed by the StatefulSet controller.
  #[serde(rename = "serviceName")]
  service_name: String,
  /// template is the object that describes the pod that will be created if insufficient replicas are detected. Each pod stamped out by the StatefulSet will fulfill this Template, but have a unique identity from the rest of the StatefulSet.
  #[serde(rename = "template")]
  template: ::models::V1PodTemplateSpec,
  /// updateStrategy indicates the StatefulSetUpdateStrategy that will be employed to update Pods in the StatefulSet when a revision is made to Template.
  #[serde(rename = "updateStrategy")]
  update_strategy: Option<::models::AppsV1beta1StatefulSetUpdateStrategy>,
  /// volumeClaimTemplates is a list of claims that pods are allowed to reference. The StatefulSet controller is responsible for mapping network identities to claims in a way that maintains the identity of a pod. Every claim in this list must have at least one matching (by name) volumeMount in one container in the template. A claim in this list takes precedence over any volumes in the template, with the same name.
  #[serde(rename = "volumeClaimTemplates")]
  volume_claim_templates: Option<Vec<::models::V1PersistentVolumeClaim>>
}

impl AppsV1beta1StatefulSetSpec {
  /// A StatefulSetSpec is the specification of a StatefulSet.
  pub fn new(service_name: String, template: ::models::V1PodTemplateSpec) -> AppsV1beta1StatefulSetSpec {
    AppsV1beta1StatefulSetSpec {
      pod_management_policy: None,
      replicas: None,
      revision_history_limit: None,
      selector: None,
      service_name: service_name,
      template: template,
      update_strategy: None,
      volume_claim_templates: None
    }
  }

  pub fn set_pod_management_policy(&mut self, pod_management_policy: String) {
    self.pod_management_policy = Some(pod_management_policy);
  }

  pub fn with_pod_management_policy(mut self, pod_management_policy: String) -> AppsV1beta1StatefulSetSpec {
    self.pod_management_policy = Some(pod_management_policy);
    self
  }

  pub fn pod_management_policy(&self) -> Option<&String> {
    self.pod_management_policy.as_ref()
  }

  pub fn reset_pod_management_policy(&mut self) {
    self.pod_management_policy = None;
  }

  pub fn set_replicas(&mut self, replicas: i32) {
    self.replicas = Some(replicas);
  }

  pub fn with_replicas(mut self, replicas: i32) -> AppsV1beta1StatefulSetSpec {
    self.replicas = Some(replicas);
    self
  }

  pub fn replicas(&self) -> Option<&i32> {
    self.replicas.as_ref()
  }

  pub fn reset_replicas(&mut self) {
    self.replicas = None;
  }

  pub fn set_revision_history_limit(&mut self, revision_history_limit: i32) {
    self.revision_history_limit = Some(revision_history_limit);
  }

  pub fn with_revision_history_limit(mut self, revision_history_limit: i32) -> AppsV1beta1StatefulSetSpec {
    self.revision_history_limit = Some(revision_history_limit);
    self
  }

  pub fn revision_history_limit(&self) -> Option<&i32> {
    self.revision_history_limit.as_ref()
  }

  pub fn reset_revision_history_limit(&mut self) {
    self.revision_history_limit = None;
  }

  pub fn set_selector(&mut self, selector: ::models::MetaV1LabelSelector) {
    self.selector = Some(selector);
  }

  pub fn with_selector(mut self, selector: ::models::MetaV1LabelSelector) -> AppsV1beta1StatefulSetSpec {
    self.selector = Some(selector);
    self
  }

  pub fn selector(&self) -> Option<&::models::MetaV1LabelSelector> {
    self.selector.as_ref()
  }

  pub fn reset_selector(&mut self) {
    self.selector = None;
  }

  pub fn set_service_name(&mut self, service_name: String) {
    self.service_name = service_name;
  }

  pub fn with_service_name(mut self, service_name: String) -> AppsV1beta1StatefulSetSpec {
    self.service_name = service_name;
    self
  }

  pub fn service_name(&self) -> &String {
    &self.service_name
  }


  pub fn set_template(&mut self, template: ::models::V1PodTemplateSpec) {
    self.template = template;
  }

  pub fn with_template(mut self, template: ::models::V1PodTemplateSpec) -> AppsV1beta1StatefulSetSpec {
    self.template = template;
    self
  }

  pub fn template(&self) -> &::models::V1PodTemplateSpec {
    &self.template
  }


  pub fn set_update_strategy(&mut self, update_strategy: ::models::AppsV1beta1StatefulSetUpdateStrategy) {
    self.update_strategy = Some(update_strategy);
  }

  pub fn with_update_strategy(mut self, update_strategy: ::models::AppsV1beta1StatefulSetUpdateStrategy) -> AppsV1beta1StatefulSetSpec {
    self.update_strategy = Some(update_strategy);
    self
  }

  pub fn update_strategy(&self) -> Option<&::models::AppsV1beta1StatefulSetUpdateStrategy> {
    self.update_strategy.as_ref()
  }

  pub fn reset_update_strategy(&mut self) {
    self.update_strategy = None;
  }

  pub fn set_volume_claim_templates(&mut self, volume_claim_templates: Vec<::models::V1PersistentVolumeClaim>) {
    self.volume_claim_templates = Some(volume_claim_templates);
  }

  pub fn with_volume_claim_templates(mut self, volume_claim_templates: Vec<::models::V1PersistentVolumeClaim>) -> AppsV1beta1StatefulSetSpec {
    self.volume_claim_templates = Some(volume_claim_templates);
    self
  }

  pub fn volume_claim_templates(&self) -> Option<&Vec<::models::V1PersistentVolumeClaim>> {
    self.volume_claim_templates.as_ref()
  }

  pub fn reset_volume_claim_templates(&mut self) {
    self.volume_claim_templates = None;
  }

}



