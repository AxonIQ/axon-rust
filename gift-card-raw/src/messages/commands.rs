use crate::messages::AxonMessage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IssueGiftCard {
    pub id: String,
    pub amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct RedeemGiftCard {
    pub id: String,
    pub amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CancelGiftCard {
    pub id: String,
}

pub enum AggregateCreationPolicy {
    Always,
    CreateIfMissing,
    Never,
}

pub trait AxonCommand {
    fn creation_policy() -> AggregateCreationPolicy;
}

impl AxonMessage for IssueGiftCard {
    fn name() -> &'static str {
        "IssueGiftCard"
    }
}

impl AxonMessage for RedeemGiftCard {
    fn name() -> &'static str {
        "RedeemGiftCard"
    }
}

impl AxonMessage for CancelGiftCard {
    fn name() -> &'static str {
        "CancelGiftCard"
    }
}

impl AxonCommand for IssueGiftCard {
    fn creation_policy() -> AggregateCreationPolicy {
        AggregateCreationPolicy::Always
    }
}

impl AxonCommand for RedeemGiftCard {
    fn creation_policy() -> AggregateCreationPolicy {
        AggregateCreationPolicy::Never
    }
}

impl AxonCommand for CancelGiftCard {
    fn creation_policy() -> AggregateCreationPolicy {
        AggregateCreationPolicy::Never
    }
}
