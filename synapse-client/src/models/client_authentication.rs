/*
 * Axon Synapse API
 *
 * API for Axon Synapse http services
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ClientAuthentication : Authentication used by synapse to call the registered handler with sensitive data stripped



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientAuthentication {
    /// Name to recognize this stored authentication
    #[serde(rename = "name")]
    pub name: String,
    /// Id of the stored authentication used by synapse to call the registered handler
    #[serde(rename = "id")]
    pub id: String,
}

impl ClientAuthentication {
    /// Authentication used by synapse to call the registered handler with sensitive data stripped
    pub fn new(name: String, id: String) -> ClientAuthentication {
        ClientAuthentication {
            name,
            id,
        }
    }
}


