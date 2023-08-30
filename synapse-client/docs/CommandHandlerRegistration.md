# CommandHandlerRegistration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**names** | **Vec<String>** |  | 
**endpoint** | **String** | Message-handling client application endpoint | 
**endpoint_type** | Option<**String**> | Type of client application endpoint - `http-raw` endpoint type is served with single raw command payload - `http-message` endpoint type is served with single command message  | [optional][default to http-raw]
**endpoint_options** | Option<[**Vec<crate::models::EndpointOption>**](EndpointOption.md)> |  | [optional]
**client_id** | Option<**String**> | Unique client application identifier | [optional]
**component_name** | Option<**String**> | Client application name | [optional]
**load_factor** | Option<**i32**> | The amount of load an Axon application would carry compared to other instances.  For example, if you have a two command handlers set up, each with a load factor of 100,  they will both carry an equal amount of load  | [optional][default to 100]
**concurrency** | Option<**i32**> | The number of concurrent commands that the command handler can handle. | [optional][default to 1]
**enabled** | Option<**bool**> | Is this command handler enabled? | [optional][default to true]
**context** | Option<**String**> | Context name | [optional][default to default]
**client_authentication_id** | Option<**String**> | Id of the stored authentication used by synapse to call the registered handler | [optional]
**server_authentication_id** | Option<**String**> | Id of the stored authentication used by synapse to contact axon server for this registration | [optional]
**last_error** | Option<**String**> | The last error occuring with this handler | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


