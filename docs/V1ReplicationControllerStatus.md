# V1ReplicationControllerStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available_replicas** | **i32** | The number of available replicas (ready for at least minReadySeconds) for this replication controller. | [optional] [default to null]
**conditions** | [**Vec<::models::V1ReplicationControllerCondition>**](io.k8s.kubernetes.pkg.api.v1.ReplicationControllerCondition.md) | Represents the latest available observations of a replication controller&#39;s current state. | [optional] [default to null]
**fully_labeled_replicas** | **i32** | The number of pods that have labels matching the labels of the pod template of the replication controller. | [optional] [default to null]
**observed_generation** | **i64** | ObservedGeneration reflects the generation of the most recently observed replication controller. | [optional] [default to null]
**ready_replicas** | **i32** | The number of ready replicas for this replication controller. | [optional] [default to null]
**replicas** | **i32** | Replicas is the most recently oberved number of replicas. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#what-is-a-replicationcontroller | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


