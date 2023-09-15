use synapse_client::apis::{configuration, Error};
use synapse_client::apis::commands_api::{send_command_message, SendCommandMessageError};
use synapse_client::models::{CommandMessage, CommandResponseMessage};

use crate::api::GiftCardCommand;

/// Send/Dispatch a command to Axon Server
pub async fn send_gift_card_command(command: GiftCardCommand, configuration: &configuration::Configuration, context: &String) -> Result<CommandResponseMessage, Error<SendCommandMessageError>> {
    send_command_message(configuration, context, Some(command.to_command_message())).await
}

/// Map to Axon CommandMessage
trait ToCommandMessage {
    fn to_command_message(&self) -> CommandMessage;
}

/// Map from domain commands of type GiftCardCommand to Axon CommandMessage
impl ToCommandMessage for GiftCardCommand {
    fn to_command_message(&self) -> CommandMessage {
        let payload = serde_json::to_value(self).unwrap();
        CommandMessage {
            name: self.payload_type(),
            routing_key: Some(self.id()),
            priority: None,
            id: None,
            meta_data: None,
            payload: Some(Some(payload)),
            payload_type: Some(self.payload_type()),
            payload_revision: None,
        }
    }
}