# PublishableEventMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**payload_type** | Option<**String**> | Type of the payload. To integrate with Axon Framework based application use fully-qualified class name | [optional][readonly]
**name** | **String** | Event name. Alias for PayloadType. To integrate with Axon Framework based application use fully-qualified class name | 
**aggregate_id** | Option<**String**> | Unique aggregate identifier | [optional]
**aggregate_type** | Option<**String**> | Aggregate type. To integrate with Axon Framework based application use simple class name | [optional]
**sequence_number** | Option<**i64**> | Sequence number | [optional]
**date_time** | Option<**String**> | Date and time | [optional]
**index** | Option<**i64**> | Global index of an event in the event store | [optional]
**id** | Option<**String**> | Unique message identifier | [optional]
**meta_data** | Option<**::std::collections::HashMap<String, String>**> | Key-value map with message meta data | [optional]
**payload** | Option<[**serde_json::Value**](.md)> | Text payload. Accepts JSON, XML, UTF-8 Text, Base64 encoded binary data | [optional]
**payload_revision** | Option<**String**> | Revision of the payload. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


