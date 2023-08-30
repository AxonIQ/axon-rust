# \AuthenticationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_client_authentication_token**](AuthenticationApi.md#create_client_authentication_token) | **POST** /v1/authentications/client/tokens | Creates the provided client authentication token
[**create_server_authentication_token**](AuthenticationApi.md#create_server_authentication_token) | **POST** /v1/authentications/server/tokens | Creates the provided server authentication token
[**delete_client_authentication_token**](AuthenticationApi.md#delete_client_authentication_token) | **DELETE** /v1/authentications/client/tokens/{authId} | Deletes the provided client authentication token
[**delete_server_authentication_token**](AuthenticationApi.md#delete_server_authentication_token) | **DELETE** /v1/authentications/server/tokens/{authId} | Deletes the provided server authentication token
[**get_client_authentication_token**](AuthenticationApi.md#get_client_authentication_token) | **GET** /v1/authentications/client/tokens/{authId} | Returns a single client authentication token
[**get_server_authentication_token**](AuthenticationApi.md#get_server_authentication_token) | **GET** /v1/authentications/server/tokens/{authId} | Returns a single server authentication token
[**list_authentication**](AuthenticationApi.md#list_authentication) | **GET** /v1/authentications | Returns a list of all authentications (client and server) of all types (token, etc.)
[**list_client_authentication_token**](AuthenticationApi.md#list_client_authentication_token) | **GET** /v1/authentications/client/tokens | Returns a list of client authentication tokens
[**list_server_authentication_token**](AuthenticationApi.md#list_server_authentication_token) | **GET** /v1/authentications/server/tokens | Returns a list of server authentication tokens
[**update_client_authentication_token**](AuthenticationApi.md#update_client_authentication_token) | **PUT** /v1/authentications/client/tokens/{authId} | Updates the provided client authentication token
[**update_server_authentication_token**](AuthenticationApi.md#update_server_authentication_token) | **PUT** /v1/authentications/server/tokens/{authId} | Updates the provided server authentication token



## create_client_authentication_token

> crate::models::ClientAuthentication create_client_authentication_token(client_authentication_token_prototype)
Creates the provided client authentication token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_authentication_token_prototype** | Option<[**ClientAuthenticationTokenPrototype**](ClientAuthenticationTokenPrototype.md)> |  |  |

### Return type

[**crate::models::ClientAuthentication**](ClientAuthentication.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_server_authentication_token

> crate::models::ServerAuthentication create_server_authentication_token(server_authentication_token_prototype)
Creates the provided server authentication token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_authentication_token_prototype** | Option<[**ServerAuthenticationTokenPrototype**](ServerAuthenticationTokenPrototype.md)> |  |  |

### Return type

[**crate::models::ServerAuthentication**](ServerAuthentication.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_client_authentication_token

> delete_client_authentication_token(auth_id)
Deletes the provided client authentication token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_id** | **String** | Authentication id | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_server_authentication_token

> delete_server_authentication_token(auth_id)
Deletes the provided server authentication token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_id** | **String** | Authentication id | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client_authentication_token

> crate::models::ClientAuthentication get_client_authentication_token(auth_id)
Returns a single client authentication token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_id** | **String** | Authentication id | [required] |

### Return type

[**crate::models::ClientAuthentication**](ClientAuthentication.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_authentication_token

> crate::models::ServerAuthentication get_server_authentication_token(auth_id)
Returns a single server authentication token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_id** | **String** | Authentication id | [required] |

### Return type

[**crate::models::ServerAuthentication**](ServerAuthentication.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_authentication

> crate::models::ListOfAuthenticationObjects list_authentication()
Returns a list of all authentications (client and server) of all types (token, etc.)

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListOfAuthenticationObjects**](ListOfAuthenticationObjects.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_client_authentication_token

> crate::models::ListOfClientAuthentication list_client_authentication_token()
Returns a list of client authentication tokens

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListOfClientAuthentication**](ListOfClientAuthentication.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_server_authentication_token

> crate::models::ListOfServerAuthentication list_server_authentication_token()
Returns a list of server authentication tokens

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListOfServerAuthentication**](ListOfServerAuthentication.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_client_authentication_token

> crate::models::ClientAuthentication update_client_authentication_token(auth_id, client_authentication_token_prototype)
Updates the provided client authentication token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_id** | **String** | Authentication id | [required] |
**client_authentication_token_prototype** | Option<[**ClientAuthenticationTokenPrototype**](ClientAuthenticationTokenPrototype.md)> |  |  |

### Return type

[**crate::models::ClientAuthentication**](ClientAuthentication.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_server_authentication_token

> crate::models::ServerAuthentication update_server_authentication_token(auth_id, server_authentication_token_prototype)
Updates the provided server authentication token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_id** | **String** | Authentication id | [required] |
**server_authentication_token_prototype** | Option<[**ServerAuthenticationTokenPrototype**](ServerAuthenticationTokenPrototype.md)> |  |  |

### Return type

[**crate::models::ServerAuthentication**](ServerAuthentication.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

