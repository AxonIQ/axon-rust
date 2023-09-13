use crate::messages::AxonMessage;
use serde::{Deserialize, Serialize};
use synapse_client::models::{QueryMessage, QueryResponseCardinality};

#[derive(Serialize, Deserialize)]
pub struct GiftCardSummary {
    pub id: String,
    pub initial_amount: u32,
    pub remaining_amount: u32,
    pub canceled: bool,
    pub issued: String,
    pub last_updated: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FetchGiftCardSummary {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FetchGiftCardSummaries {
    pub limit: i32,
    pub offset: i32,
}

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
                let fetch_gift_card_summary: FetchGiftCardSummary =
                    serde_json::from_value(value).unwrap();
                Some(GiftCardQuery::One(fetch_gift_card_summary))
            }
            "FetchGiftCardSummaries" => {
                let fetch_gift_card_summaries: FetchGiftCardSummaries =
                    serde_json::from_value(value).unwrap();
                Some(GiftCardQuery::Multiple(fetch_gift_card_summaries))
            }
            _ => None,
        }
    }
}

pub fn to_query_message<T>(name: &str, query: &T) -> QueryMessage
where
    T: Serialize,
{
    QueryMessage {
        name: String::from(name),
        number_of_responses: Some(1),
        response_cardinality: Some(QueryResponseCardinality::Single),
        response_type: Some(Some(String::from("json"))),
        response_type_encoding: Some(String::from("application/json")),
        id: None,
        meta_data: None,
        payload: Some(Some(serde_json::to_value(query).unwrap())),
        payload_type: None,
        payload_revision: None,
    }
}
