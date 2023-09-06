/*
 * Axon Synapse API
 *
 * API for Axon Synapse http services
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListOfQueryHandlers : List of query handlers



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListOfQueryHandlers {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::QueryHandler>>,
}

impl ListOfQueryHandlers {
    /// List of query handlers
    pub fn new() -> ListOfQueryHandlers {
        ListOfQueryHandlers {
            items: None,
        }
    }
}

