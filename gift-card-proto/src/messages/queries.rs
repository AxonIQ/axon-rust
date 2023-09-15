use crate::gift_card::queries::{FetchGiftCardSummaries, FetchGiftCardSummary};
use crate::messages::{message_to_payload, value_to_message, AxonMessage};
use prost::Message;
use synapse_client::models::{QueryMessage, QueryResponseCardinality};

impl AxonMessage for FetchGiftCardSummary {
    fn name() -> &'static str {
        "FetchGiftCardSummary"
    }
}

impl AxonMessage for FetchGiftCardSummaries {
    fn name() -> &'static str {
        "FetchGiftCardSummaries"
    }
}

#[derive(Debug)]
pub enum GiftCardQuery {
    One(FetchGiftCardSummary),
    Multiple(FetchGiftCardSummaries),
}

pub trait ContainsGiftCardQuery {
    fn get_gift_card_query(&self) -> Option<GiftCardQuery>;
}

impl ContainsGiftCardQuery for QueryMessage {
    fn get_gift_card_query(&self) -> Option<GiftCardQuery> {
        let value = self.payload.clone().unwrap().unwrap();
        match self.name.as_str() {
            "FetchGiftCardSummary" => {
                let fetch_gift_card_summary: FetchGiftCardSummary = value_to_message(value);
                Some(GiftCardQuery::One(fetch_gift_card_summary))
            }
            "FetchGiftCardSummaries" => {
                let fetch_gift_card_summaries: FetchGiftCardSummaries = value_to_message(value);
                Some(GiftCardQuery::Multiple(fetch_gift_card_summaries))
            }
            _ => None,
        }
    }
}

pub fn to_query_message<T>(name: &str, query: &T) -> QueryMessage
where
    T: Message,
{
    let payload = message_to_payload(query);
    QueryMessage {
        name: String::from(name),
        number_of_responses: Some(1),
        response_cardinality: Some(QueryResponseCardinality::Single),
        response_type: Some(Some(String::from("json"))),
        response_type_encoding: Some(String::from("application/octet-stream")),
        id: None,
        meta_data: None,
        payload,
        payload_type: None,
        payload_revision: None,
    }
}
