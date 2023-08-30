# \EventHandlersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**enable_event_handler**](EventHandlersApi.md#enable_event_handler) | **PATCH** /v1/contexts/{context}/handlers/events/{eventHandlerId} | Enables/disables the event handler
[**event_endpoint_types**](EventHandlersApi.md#event_endpoint_types) | **GET** /v1/handlers/events/endpointTypes | Returns a list of all endpoint types for events
[**get_event_handler**](EventHandlersApi.md#get_event_handler) | **GET** /v1/contexts/{context}/handlers/events/{eventHandlerId} | Returns the event handler
[**list_event_handlers**](EventHandlersApi.md#list_event_handlers) | **GET** /v1/contexts/{context}/handlers/events | Returns a list of event handlers
[**list_event_handlers_of_all_contexts**](EventHandlersApi.md#list_event_handlers_of_all_contexts) | **GET** /v1/handlers/events | Returns a list of event handlers of all contexts
[**register_event_handler**](EventHandlersApi.md#register_event_handler) | **POST** /v1/contexts/{context}/handlers/events | Registers a new event handler
[**replace_event_handler**](EventHandlersApi.md#replace_event_handler) | **PUT** /v1/contexts/{context}/handlers/events/{eventHandlerId} | Created or replaces the event handler
[**unregister_event_handler**](EventHandlersApi.md#unregister_event_handler) | **DELETE** /v1/contexts/{context}/handlers/events/{eventHandlerId} | Unregisters the event handler



## enable_event_handler

> enable_event_handler(context, event_handler_id, enabled)
Enables/disables the event handler

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**event_handler_id** | **String** | Event handler id | [required] |
**enabled** | **bool** | true to enable the handler, false to disable the handler | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## event_endpoint_types

> Vec<crate::models::EndpointType> event_endpoint_types()
Returns a list of all endpoint types for events

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


## get_event_handler

> crate::models::EventHandler get_event_handler(context, event_handler_id)
Returns the event handler

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**event_handler_id** | **String** | Event handler id | [required] |

### Return type

[**crate::models::EventHandler**](EventHandler.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_event_handlers

> crate::models::ListOfEventHandlers list_event_handlers(context)
Returns a list of event handlers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]

### Return type

[**crate::models::ListOfEventHandlers**](ListOfEventHandlers.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_event_handlers_of_all_contexts

> crate::models::ListOfEventHandlers list_event_handlers_of_all_contexts()
Returns a list of event handlers of all contexts

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListOfEventHandlers**](ListOfEventHandlers.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_event_handler

> crate::models::EventHandler register_event_handler(context, event_handler_registration)
Registers a new event handler

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**event_handler_registration** | Option<[**EventHandlerRegistration**](EventHandlerRegistration.md)> |  |  |

### Return type

[**crate::models::EventHandler**](EventHandler.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_event_handler

> crate::models::EventHandler replace_event_handler(context, event_handler_id, event_handler_registration)
Created or replaces the event handler

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**event_handler_id** | **String** | Event handler id | [required] |
**event_handler_registration** | Option<[**EventHandlerRegistration**](EventHandlerRegistration.md)> |  |  |

### Return type

[**crate::models::EventHandler**](EventHandler.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unregister_event_handler

> unregister_event_handler(context, event_handler_id)
Unregisters the event handler

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**event_handler_id** | **String** | Event handler id | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

