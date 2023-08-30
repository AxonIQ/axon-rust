# \AggregateApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read_aggregate_events**](AggregateApi.md#read_aggregate_events) | **GET** /v1/contexts/{context}/aggregate/{aggregateId}/events | Returns all events for the aggregate



## read_aggregate_events

> crate::models::ListOfEventMessages read_aggregate_events(context, aggregate_id)
Returns all events for the aggregate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**aggregate_id** | **String** | The id of the aggregate | [required] |

### Return type

[**crate::models::ListOfEventMessages**](ListOfEventMessages.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

