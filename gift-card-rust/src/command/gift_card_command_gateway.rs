use synapse_client::apis::commands_api::{send_command_message, SendCommandMessageError};
use synapse_client::apis::{configuration, Error};
use synapse_client::models::{CommandMessage, CommandResponseMessage};

use crate::gift_card_api::GiftCardCommand;

/// Send/Dispatch a command to Axon Server
pub async fn send_gift_card_command(
    command: GiftCardCommand,
    configuration: &configuration::Configuration,
    context: &str,
) -> Result<CommandResponseMessage, Error<SendCommandMessageError>> {
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
            priority: Some(1),
            id: Some(uuid::Uuid::new_v4().to_string()),
            meta_data: None,
            payload: Some(Some(payload)),
            payload_type: Some(self.payload_type()),
            payload_revision: None,
        }
    }
}
