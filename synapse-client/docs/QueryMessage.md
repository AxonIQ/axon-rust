# QueryMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Query name. To integrate with Axon Framework based application use fully-qualified class name | 
**number_of_responses** | Option<**i32**> | The number of query handlers that should answer the query. For direct query set to `1` | [optional][default to 1]
**response_cardinality** | Option<[**crate::models::QueryResponseCardinality**](QueryResponseCardinality.md)> |  | [optional]
**response_type** | Option<**String**> | Expected response type required when performing a query. Specific for queries that are  handled by Axon Framework applications.  | [optional][default to java.lang.Object]
**response_type_encoding** | Option<**String**> | Specifies the way that the Axon Framework application expects the response type to be  serialized. Values `application/json` and `application/xml` are valid for Axon Framework.  | [optional]
**id** | Option<**String**> | Unique message identifier | [optional]
**meta_data** | Option<**::std::collections::HashMap<String, String>**> | Key-value map with message meta data | [optional]
**payload** | Option<[**serde_json::Value**](.md)> | Text payload. Accepts JSON, XML, UTF-8 Text, Base64 encoded binary data | [optional]
**payload_type** | Option<**String**> | Type of the payload. To integrate with Axon Framework based application use fully-qualified class name | [optional]
**payload_revision** | Option<**String**> | Revision of the payload. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


