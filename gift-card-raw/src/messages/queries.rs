use crate::messages::AxonMessage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CardSummary {
    pub id: String,
    pub initial_amount: u32,
    pub remaining_amount: u32,
    pub canceled: bool,
    pub issued: String,
    pub last_updated: String,
}

#[derive(Serialize, Deserialize)]
pub struct FetchGiftCardSummary {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
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
