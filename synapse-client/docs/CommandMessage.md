# CommandMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Command name. To integrate with Axon Framework based application use fully-qualified class name | 
**routing_key** | Option<**String**> | Key used for consistent command routing among command handling instances. | [optional]
**priority** | Option<**i32**> | Relative priority of the command | [optional]
**id** | Option<**String**> | Unique message identifier | [optional]
**meta_data** | Option<**::std::collections::HashMap<String, String>**> | Key-value map with message meta data | [optional]
**payload** | Option<[**serde_json::Value**](.md)> | Text payload. Accepts JSON, XML, UTF-8 Text, Base64 encoded binary data | [optional]
**payload_type** | Option<**String**> | Type of the payload. To integrate with Axon Framework based application use fully-qualified class name | [optional]
**payload_revision** | Option<**String**> | Revision of the payload. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


