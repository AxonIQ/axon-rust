/*
 * Axon Synapse API
 *
 * API for Axon Synapse http services
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// QueryResponseCardinality : Do I expect the query handler return a single item in the response or a list of items

/// Do I expect the query handler return a single item in the response or a list of items
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum QueryResponseCardinality {
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "multiple")]
    Multiple,

}

impl ToString for QueryResponseCardinality {
    fn to_string(&self) -> String {
        match self {
            Self::Single => String::from("single"),
            Self::Multiple => String::from("multiple"),
        }
    }
}

impl Default for QueryResponseCardinality {
    fn default() -> QueryResponseCardinality {
        Self::Single
    }
}



