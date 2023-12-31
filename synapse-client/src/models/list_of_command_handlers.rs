/*
 * Axon Synapse API
 *
 * API for Axon Synapse http services
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListOfCommandHandlers : List of command handlers



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListOfCommandHandlers {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::CommandHandler>>,
}

impl ListOfCommandHandlers {
    /// List of command handlers
    pub fn new() -> ListOfCommandHandlers {
        ListOfCommandHandlers {
            items: None,
        }
    }
}


