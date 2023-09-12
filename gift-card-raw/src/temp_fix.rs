use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventMessage {
    /// Event name. Alias for PayloadType. To integrate with Axon Framework based application use fully-qualified class name
    #[serde(rename = "name")]
    pub name: String,
    /// Unique aggregate identifier
    #[serde(rename = "aggregateId", skip_serializing_if = "Option::is_none")]
    pub aggregate_id: Option<String>,
    /// Aggregate type. To integrate with Axon Framework based application use simple class name
    #[serde(rename = "aggregateType", skip_serializing_if = "Option::is_none")]
    pub aggregate_type: Option<String>,
    /// Sequence number
    #[serde(rename = "sequenceNumber", skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<i64>,
    /// Date and time
    #[serde(rename = "dateTime", skip_serializing_if = "Option::is_none")]
    pub date_time: Option<f64>,
    /// Global index of an event in the event store
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// Unique message identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Key-value map with message meta data
    #[serde(rename = "metaData", skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<::std::collections::HashMap<String, String>>,
    /// Text payload. Accepts JSON, XML, UTF-8 Text, Base64 encoded binary data
    #[serde(
        rename = "payload",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub payload: Option<Option<serde_json::Value>>,
    /// Type of the payload. To integrate with Axon Framework based application use fully-qualified class name
    #[serde(rename = "payloadType", skip_serializing_if = "Option::is_none")]
    pub payload_type: Option<String>,
    /// Revision of the payload.
    #[serde(rename = "payloadRevision", skip_serializing_if = "Option::is_none")]
    pub payload_revision: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventList {
    #[serde(rename = "items", default, skip_serializing_if = "Option::is_none")]
    pub(crate) items: Option<Vec<EventMessage>>,
}
