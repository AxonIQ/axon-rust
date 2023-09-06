/*
 * Axon Synapse API
 *
 * API for Axon Synapse http services
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AuthenticationObject : Authentication used by synapse to contact axon server on behalf of another application



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticationObject {
    /// Intended audience of this authentication object (client/server)
    #[serde(rename = "audience")]
    pub audience: String,
    /// Type of this authentication object
    #[serde(rename = "type")]
    pub r#type: String,
    /// Name to recognize this stored authentication
    #[serde(rename = "name")]
    pub name: String,
    /// Unique ID of the authentication object
    #[serde(rename = "id")]
    pub id: String,
    /// Indicates whether this authentication object is readable
    #[serde(rename = "readable", skip_serializing_if = "Option::is_none")]
    pub readable: Option<bool>,
}

impl AuthenticationObject {
    /// Authentication used by synapse to contact axon server on behalf of another application
    pub fn new(audience: String, r#type: String, name: String, id: String) -> AuthenticationObject {
        AuthenticationObject {
            audience,
            r#type,
            name,
            id,
            readable: None,
        }
    }
}

