use crate::gift_card::commands::{CancelGiftCard, IssueGiftCard, RedeemGiftCard};
use crate::messages::{message_to_payload, value_to_message, AxonMessage};
use prost::Message;
use synapse_client::models::CommandMessage;
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
}

pub trait ContainsGiftCardCommand {
    fn get_gift_card_command(&self) -> Option<GiftCardCommand>;
}

impl ContainsGiftCardCommand for CommandMessage {
    fn get_gift_card_command(&self) -> Option<GiftCardCommand> {
        let value = self.payload.clone().unwrap().unwrap();
        match self.name.as_str() {
            "IssueGiftCard" => {
                let issue_gift_card: IssueGiftCard = value_to_message(value);
                Some(GiftCardCommand::Issue(issue_gift_card))
            }
            "RedeemGiftCard" => {
                let redeem_gift_card: RedeemGiftCard = value_to_message(value);
                Some(GiftCardCommand::Redeem(redeem_gift_card))
            }
            "CancelGiftCard" => {
                let cancel_gift_card: CancelGiftCard = value_to_message(value);
                Some(GiftCardCommand::Cancel(cancel_gift_card))
            }
            _ => None,
        }
    }
}

pub fn to_command_message<T>(name: &str, routing_key: Option<String>, command: &T) -> CommandMessage
where
    T: Message,
{
    let payload = message_to_payload(command);
    CommandMessage {
        name: String::from(name),
        routing_key,
        priority: None,
        id: None,
        meta_data: None,
        payload,
        payload_type: None,
        payload_revision: None,
    }
}
