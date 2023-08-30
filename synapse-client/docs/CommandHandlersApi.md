# \CommandHandlersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**command_endpoint_types**](CommandHandlersApi.md#command_endpoint_types) | **GET** /v1/handlers/commands/endpointTypes | Returns a list of all endpoint types for commands
[**enable_command_handler**](CommandHandlersApi.md#enable_command_handler) | **PATCH** /v1/contexts/{context}/handlers/commands/{commandHandlerId} | Enables/disables the command handler
[**get_command_handler**](CommandHandlersApi.md#get_command_handler) | **GET** /v1/contexts/{context}/handlers/commands/{commandHandlerId} | Returns the command handler
[**list_command_handlers**](CommandHandlersApi.md#list_command_handlers) | **GET** /v1/contexts/{context}/handlers/commands | Returns a list of command handlers
[**list_command_handlers_all_contexts**](CommandHandlersApi.md#list_command_handlers_all_contexts) | **GET** /v1/handlers/commands | Returns a list of command handlers of all contexts
[**register_command_handler**](CommandHandlersApi.md#register_command_handler) | **POST** /v1/contexts/{context}/handlers/commands | Registers a new command handler
[**replace_command_handler**](CommandHandlersApi.md#replace_command_handler) | **PUT** /v1/contexts/{context}/handlers/commands/{commandHandlerId} | Creates or replaces the command handler
[**unregister_command_handler**](CommandHandlersApi.md#unregister_command_handler) | **DELETE** /v1/contexts/{context}/handlers/commands/{commandHandlerId} | Unregisters the command handler



## command_endpoint_types

> Vec<crate::models::EndpointType> command_endpoint_types()
Returns a list of all endpoint types for commands

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


## enable_command_handler

> enable_command_handler(context, command_handler_id, enabled)
Enables/disables the command handler

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**command_handler_id** | **String** | Command handler id | [required] |
**enabled** | **bool** | true to enable the handler, false to disable the handler | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_command_handler

> crate::models::CommandHandler get_command_handler(context, command_handler_id)
Returns the command handler

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**command_handler_id** | **String** | Command handler id | [required] |

### Return type

[**crate::models::CommandHandler**](CommandHandler.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_command_handlers

> crate::models::ListOfCommandHandlers list_command_handlers(context)
Returns a list of command handlers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]

### Return type

[**crate::models::ListOfCommandHandlers**](ListOfCommandHandlers.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_command_handlers_all_contexts

> crate::models::ListOfCommandHandlers list_command_handlers_all_contexts()
Returns a list of command handlers of all contexts

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListOfCommandHandlers**](ListOfCommandHandlers.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_command_handler

> crate::models::CommandHandler register_command_handler(context, command_handler_registration)
Registers a new command handler

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**command_handler_registration** | Option<[**CommandHandlerRegistration**](CommandHandlerRegistration.md)> |  |  |

### Return type

[**crate::models::CommandHandler**](CommandHandler.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_command_handler

> crate::models::CommandHandler replace_command_handler(context, command_handler_id, command_handler_registration)
Creates or replaces the command handler

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**command_handler_id** | **String** | Command handler id | [required] |
**command_handler_registration** | Option<[**CommandHandlerRegistration**](CommandHandlerRegistration.md)> |  |  |

### Return type

[**crate::models::CommandHandler**](CommandHandler.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unregister_command_handler

> unregister_command_handler(context, command_handler_id)
Unregisters the command handler

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**command_handler_id** | **String** | Command handler id | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

