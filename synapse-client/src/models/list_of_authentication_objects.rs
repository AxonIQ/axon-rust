/*
 * Axon Synapse API
 *
 * API for Axon Synapse http services
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListOfAuthenticationObjects : List of authentication objects



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListOfAuthenticationObjects {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::AuthenticationObject>>,
}

impl ListOfAuthenticationObjects {
    /// List of authentication objects
    pub fn new() -> ListOfAuthenticationObjects {
        ListOfAuthenticationObjects {
            items: None,
        }
    }
}


