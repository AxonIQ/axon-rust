# \QueryHandlersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**enable_query_handler**](QueryHandlersApi.md#enable_query_handler) | **PATCH** /v1/contexts/{context}/handlers/queries/{queryHandlerId} | Enables/disables the query handler
[**get_query_handler**](QueryHandlersApi.md#get_query_handler) | **GET** /v1/contexts/{context}/handlers/queries/{queryHandlerId} | Returns the query handler
[**list_query_handlers**](QueryHandlersApi.md#list_query_handlers) | **GET** /v1/contexts/{context}/handlers/queries | Returns a list of query handlers
[**list_query_handlers_all_contexts**](QueryHandlersApi.md#list_query_handlers_all_contexts) | **GET** /v1/handlers/queries | Returns a list of query handlers of all contexts
[**query_endpoint_types**](QueryHandlersApi.md#query_endpoint_types) | **GET** /v1/handlers/queries/endpointTypes | Returns a list of all endpoint types for queries
[**register_query_handler**](QueryHandlersApi.md#register_query_handler) | **POST** /v1/contexts/{context}/handlers/queries | Registers a new query handler
[**replace_query_handler**](QueryHandlersApi.md#replace_query_handler) | **PUT** /v1/contexts/{context}/handlers/queries/{queryHandlerId} | Creates or replaces the query handler
[**unregister_query_handler**](QueryHandlersApi.md#unregister_query_handler) | **DELETE** /v1/contexts/{context}/handlers/queries/{queryHandlerId} | Unregisters the query handler



## enable_query_handler

> enable_query_handler(context, query_handler_id, enabled)
Enables/disables the query handler

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**query_handler_id** | **String** | Query handler id | [required] |
**enabled** | **bool** | true to enable the handler, false to disable the handler | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_query_handler

> crate::models::QueryHandler get_query_handler(context, query_handler_id)
Returns the query handler

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**query_handler_id** | **String** | Query handler id | [required] |

### Return type

[**crate::models::QueryHandler**](QueryHandler.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_query_handlers

> crate::models::ListOfQueryHandlers list_query_handlers(context)
Returns a list of query handlers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]

### Return type

[**crate::models::ListOfQueryHandlers**](ListOfQueryHandlers.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_query_handlers_all_contexts

> crate::models::ListOfQueryHandlers list_query_handlers_all_contexts()
Returns a list of query handlers of all contexts

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListOfQueryHandlers**](ListOfQueryHandlers.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_endpoint_types

> Vec<crate::models::EndpointType> query_endpoint_types()
Returns a list of all endpoint types for queries

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::EndpointType>**](EndpointType.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_query_handler

> crate::models::QueryHandler register_query_handler(context, query_handler_registration)
Registers a new query handler

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**query_handler_registration** | Option<[**QueryHandlerRegistration**](QueryHandlerRegistration.md)> |  |  |

### Return type

[**crate::models::QueryHandler**](QueryHandler.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_query_handler

> crate::models::QueryHandler replace_query_handler(context, query_handler_id, query_handler_registration)
Creates or replaces the query handler

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**query_handler_id** | **String** | Query handler id | [required] |
**query_handler_registration** | Option<[**QueryHandlerRegistration**](QueryHandlerRegistration.md)> |  |  |

### Return type

[**crate::models::QueryHandler**](QueryHandler.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unregister_query_handler

> unregister_query_handler(context, query_handler_id)
Unregisters the query handler

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**query_handler_id** | **String** | Query handler id | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

