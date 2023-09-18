use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, State};
use synapse_client::apis::configuration;
use synapse_client::models::{CommandResponseMessage, QueryResponseMessage};

use crate::command::gift_card_command_gateway::send_gift_card_command;
use crate::gift_card_api::{GiftCardCommand, GiftCardQuery};
use crate::query::gift_card_query_gateway::send_gift_card_query;

#[post("/gift_card_commands", format = "application/json", data = "<command>")]
pub async fn gift_card_commands(
    command: Json<GiftCardCommand>,
    configuration: &State<configuration::Configuration>,
    context: &State<String>,
) -> Result<Json<CommandResponseMessage>, Status> {
    let command = command.into_inner();
    let response = send_gift_card_command(command, configuration, context).await;
    response
        .map(Json)
        .map_err(|_err| Status::InternalServerError)
}

#[post("/gift_card_queries", format = "application/json", data = "<query>")]
pub async fn gift_card_queries(
    query: Json<GiftCardQuery>,
    configuration: &State<configuration::Configuration>,
    context: &State<String>,
) -> Result<Json<QueryResponseMessage>, Status> {
    let query = query.into_inner();
    let response = send_gift_card_query(query, configuration, context).await;
    response
        .map(Json)
        .map_err(|_err| Status::InternalServerError)
}
