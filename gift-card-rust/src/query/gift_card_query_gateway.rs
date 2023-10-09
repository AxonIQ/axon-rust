use synapse_client::apis::queries_api::{query_message, QueryMessageError};
use synapse_client::apis::{configuration, Error};
use synapse_client::models::{QueryMessage, QueryResponseMessage};

use crate::gift_card_api::GiftCardQuery;

/// Send/Dispatch a query to Axon Server
pub async fn send_gift_card_query(
    query: GiftCardQuery,
    configuration: &configuration::Configuration,
    context: &str,
) -> Result<QueryResponseMessage, Error<QueryMessageError>> {
    query_message(configuration, context, Some(query.to_query_message())).await
}

/// Map to Axon QueryMessage
trait ToQueryMessage {
    fn to_query_message(&self) -> QueryMessage;
}

/// Map from domain commands of type GiftCardCommand to Axon CommandMessage
impl ToQueryMessage for GiftCardQuery {
    fn to_query_message(&self) -> QueryMessage {
        let payload = serde_json::to_value(self).unwrap();
        QueryMessage {
            name: self.payload_type(),
            number_of_responses: None,
            response_cardinality: None,
            response_type: None,
            response_type_encoding: None,
            id: None,
            meta_data: None,
            payload: Some(Some(payload)),
            payload_type: Some(self.payload_type()),
            payload_revision: None,
        }
    }
}
