/*
 * Axon Synapse API
 *
 * API for Axon Synapse http services
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListOfEventHandlers : List of event handlers



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListOfEventHandlers {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::EventHandler>>,
}

impl ListOfEventHandlers {
    /// List of event handlers
    pub fn new() -> ListOfEventHandlers {
        ListOfEventHandlers {
            items: None,
        }
    }
}


