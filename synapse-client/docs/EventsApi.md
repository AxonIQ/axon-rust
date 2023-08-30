# \EventsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**publish_event**](EventsApi.md#publish_event) | **POST** /v1/contexts/{context}/events/{eventName} | Publishes the event
[**publish_event_message**](EventsApi.md#publish_event_message) | **POST** /v1/contexts/{context}/events | Publishes the event message



## publish_event

> publish_event(context, event_name, axon_iq_payload_revision, axon_iq_aggregate_id, axon_iq_aggregate_type, axon_iq_sequence_number, axon_iq_date_time, body)
Publishes the event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**event_name** | **String** | Event name | [required] |
**axon_iq_payload_revision** | Option<**String**> | Payload revision |  |
**axon_iq_aggregate_id** | Option<**String**> | Aggregate identifier |  |
**axon_iq_aggregate_type** | Option<**String**> | Aggregate type |  |
**axon_iq_sequence_number** | Option<**i64**> | Sequence number |  |
**axon_iq_date_time** | Option<**String**> | Date and time of the event |  |
**body** | Option<**std::path::PathBuf**> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_event_message

> publish_event_message(context, publishable_event_message)
Publishes the event message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**publishable_event_message** | Option<[**PublishableEventMessage**](PublishableEventMessage.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json, application/vnd.axoniq.event.list+json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

