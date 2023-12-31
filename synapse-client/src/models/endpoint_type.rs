/*
 * Axon Synapse API
 *
 * API for Axon Synapse http services
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EndpointType : Endpoint Type



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointType {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl EndpointType {
    /// Endpoint Type
    pub fn new(name: String) -> EndpointType {
        EndpointType {
            name,
            description: None,
        }
    }
}


