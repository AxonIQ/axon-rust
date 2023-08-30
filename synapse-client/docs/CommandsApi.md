# \CommandsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**send_command**](CommandsApi.md#send_command) | **POST** /v1/contexts/{context}/commands/{commandName} | Sends the command
[**send_command_message**](CommandsApi.md#send_command_message) | **POST** /v1/contexts/{context}/commands | Sends the command message



## send_command

> std::path::PathBuf send_command(context, command_name, axon_iq_payload_type, axon_iq_payload_revision, axon_iq_priority, axon_iq_routing_key, body)
Sends the command

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**command_name** | **String** | Command name | [required] |
**axon_iq_payload_type** | Option<**String**> | Payload type |  |
**axon_iq_payload_revision** | Option<**String**> | Payload revision |  |
**axon_iq_priority** | Option<**i32**> | Command priority |  |
**axon_iq_routing_key** | Option<**String**> | Command routing key |  |
**body** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_command_message

> crate::models::CommandResponseMessage send_command_message(context, command_message)
Sends the command message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context name | [required] |[default to default]
**command_message** | Option<[**CommandMessage**](CommandMessage.md)> |  |  |

### Return type

[**crate::models::CommandResponseMessage**](CommandResponseMessage.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

