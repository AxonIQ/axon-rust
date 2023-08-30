/*
 * Axon Synapse API
 *
 * API for Axon Synapse http services
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// QueryMessage : Query message



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryMessage {
    /// Query name. To integrate with Axon Framework based application use fully-qualified class name
    #[serde(rename = "name")]
    pub name: String,
    /// The number of query handlers that should answer the query. For direct query set to `1`
    #[serde(rename = "numberOfResponses", skip_serializing_if = "Option::is_none")]
    pub number_of_responses: Option<i32>,
    #[serde(rename = "responseCardinality", skip_serializing_if = "Option::is_none")]
    pub response_cardinality: Option<crate::models::QueryResponseCardinality>,
    /// Expected response type required when performing a query. Specific for queries that are  handled by Axon Framework applications. 
    #[serde(rename = "responseType", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub response_type: Option<Option<String>>,
    /// Specifies the way that the Axon Framework application expects the response type to be  serialized. Values `application/json` and `application/xml` are valid for Axon Framework. 
    #[serde(rename = "responseTypeEncoding", skip_serializing_if = "Option::is_none")]
    pub response_type_encoding: Option<String>,
    /// Unique message identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Key-value map with message meta data
    #[serde(rename = "metaData", skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<::std::collections::HashMap<String, String>>,
    /// Text payload. Accepts JSON, XML, UTF-8 Text, Base64 encoded binary data
    #[serde(rename = "payload", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Option<serde_json::Value>>,
    /// Type of the payload. To integrate with Axon Framework based application use fully-qualified class name
    #[serde(rename = "payloadType", skip_serializing_if = "Option::is_none")]
    pub payload_type: Option<String>,
    /// Revision of the payload.
    #[serde(rename = "payloadRevision", skip_serializing_if = "Option::is_none")]
    pub payload_revision: Option<String>,
}

impl QueryMessage {
    /// Query message
    pub fn new(name: String) -> QueryMessage {
        QueryMessage {
            name,
            number_of_responses: None,
            response_cardinality: None,
            response_type: None,
            response_type_encoding: None,
            id: None,
            meta_data: None,
            payload: None,
            payload_type: None,
            payload_revision: None,
        }
    }
}


