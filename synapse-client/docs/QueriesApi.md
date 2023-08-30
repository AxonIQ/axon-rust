# \QueriesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**query**](QueriesApi.md#query) | **POST** /v1/contexts/{context}/queries/{queryName} | Issues the query
[**query_message**](QueriesApi.md#query_message) | **POST** /v1/contexts/{context}/queries | Issues the query message



## query

> std::path::PathBuf query(context, query_name, axon_iq_payload_type, axon_iq_payload_revision, axon_iq_response_type, axon_iq_response_type_encoding, axon_iq_response_cardinality, axon_iq_number_of_responders, body)
Issues the query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**query_name** | **String** | Query name | [required] |
**axon_iq_payload_type** | Option<**String**> | Payload type |  |
**axon_iq_payload_revision** | Option<**String**> | Payload revision |  |
**axon_iq_response_type** | Option<**String**> | Query response type |  |[default to java.lang.Object]
**axon_iq_response_type_encoding** | Option<**String**> | Query response type encoding |  |
**axon_iq_response_cardinality** | Option<**String**> | Query response type |  |[default to single]
**axon_iq_number_of_responders** | Option<**i32**> | Query response type |  |[default to 1]
**body** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_message

> crate::models::QueryResponseMessage query_message(context, query_message)
Issues the query message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**query_message** | Option<[**QueryMessage**](QueryMessage.md)> |  |  |

### Return type

[**crate::models::QueryResponseMessage**](QueryResponseMessage.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

