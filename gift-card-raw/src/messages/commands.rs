use crate::messages::AxonMessage;
use serde::{Deserialize, Serialize};
use synapse_client::models::CommandMessage;

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueGiftCard {
    pub id: String,
    pub amount: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RedeemGiftCard {
    pub id: String,
    pub amount: u32,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Debug)]
pub enum GiftCardCommand {
    Issue(IssueGiftCard),
    Redeem(RedeemGiftCard),
    Cancel(CancelGiftCard),
}

impl GiftCardCommand {
    pub fn get_aggregate_id(&self) -> String {
        match self {
            GiftCardCommand::Issue(i) => i.id.clone(),
            GiftCardCommand::Redeem(r) => r.id.clone(),
            GiftCardCommand::Cancel(c) => c.id.clone(),
        }
    }
    pub fn get_creation_policy(&self) -> AggregateCreationPolicy {
        match self {
            GiftCardCommand::Issue(_) => IssueGiftCard::creation_policy(),
            GiftCardCommand::Redeem(_) => RedeemGiftCard::creation_policy(),
            GiftCardCommand::Cancel(_) => CancelGiftCard::creation_policy(),
        }
    }
}

pub trait ContainsGiftCardCommand {
    fn get_gift_card_command(&self) -> Option<GiftCardCommand>;
}

impl ContainsGiftCardCommand for CommandMessage {
    fn get_gift_card_command(&self) -> Option<GiftCardCommand> {
        let value = self.payload.clone().unwrap().unwrap();
        match self.name.as_str() {
            "IssueGiftCard" => {
                let issue_gift_card: IssueGiftCard = serde_json::from_value(value).unwrap();
                Some(GiftCardCommand::Issue(issue_gift_card))
            }
            "RedeemGiftCard" => {
                let redeem_gift_card: RedeemGiftCard = serde_json::from_value(value).unwrap();
                Some(GiftCardCommand::Redeem(redeem_gift_card))
            }
            "CancelGiftCard" => {
                let cancel_gift_card: CancelGiftCard = serde_json::from_value(value).unwrap();
                Some(GiftCardCommand::Cancel(cancel_gift_card))
            }
            _ => None,
        }
    }
}

pub fn to_command_message<T>(name: &str, routing_key: Option<String>, command: &T) -> CommandMessage
where
    T: Serialize,
{
    CommandMessage {
        name: String::from(name),
        routing_key,
        priority: None,
        id: None,
        meta_data: None,
        payload: Some(Some(serde_json::to_value(command).unwrap())),
        payload_type: None,
        payload_revision: None,
    }
}
