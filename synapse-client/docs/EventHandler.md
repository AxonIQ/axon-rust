# EventHandler

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique event handler identifier | 
**synapse_instance_id** | Option<**String**> | Unique identifier of a synapse instance | [optional]
**names** | **Vec<String>** |  | 
**endpoint** | **String** | Message-handling client application endpoint | 
**endpoint_type** | Option<**String**> | Type of client application endpoint - `http-raw` endpoint type is served with single raw event payload - `http-message` endpoint type is served with single event message - `http-list-of-messages` endpoint type is served with batched event messages  | [optional][default to http-raw]
**endpoint_options** | Option<[**Vec<crate::models::EndpointOption>**](EndpointOption.md)> |  | [optional]
**client_id** | Option<**String**> | Unique client application identifier | [optional]
**component_name** | Option<**String**> | Client application name | [optional]
**batch_size** | Option<**i32**> | Number of events to send  | [optional][default to 1]
**start** | Option<**i64**> | Position in the event stream from which the event handler receives event - `0` indicates the oldest (very first) position (tail of the stream) - `-1` indicates the latest (newest) position (head of the stream)  | [optional][default to 0]
**enabled** | Option<**bool**> | Is this command handler enabled? | [optional][default to true]
**context** | Option<**String**> | Context name | [optional][default to default]
**client_authentication_id** | Option<**String**> | Id of the stored authentication used by synapse to call the registered handler | [optional]
**server_authentication_id** | Option<**String**> | Id of the stored authentication used by synapse to contact axon server for this registration | [optional]
**last_error** | Option<**String**> | The last error occuring with this handler | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


