/*
 * Axon Synapse API
 *
 * API for Axon Synapse http services
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Message : Generic message



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
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

impl Message {
    /// Generic message
    pub fn new() -> Message {
        Message {
            id: None,
            meta_data: None,
            payload: None,
            payload_type: None,
            payload_revision: None,
        }
    }
}


