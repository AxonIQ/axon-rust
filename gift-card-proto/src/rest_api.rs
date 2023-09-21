use crate::gift_card::commands::{CancelGiftCard, IssueGiftCard, RedeemGiftCard};
use crate::gift_card::queries::{
    FetchGiftCardSummaries, FetchGiftCardSummary, MultipleGiftCards, OneGiftCard,
};
use crate::messages::commands::to_command_message;
use crate::messages::queries::to_query_message;
use crate::messages::{value_to_message, AxonMessage};
use crate::{CONFIGURATION, CONTEXT};
use serde_derive::Serialize;
use synapse_client::apis::commands_api::send_command_message;
use synapse_client::apis::queries_api::query_message;
use synapse_client::apis::Error::{Io, Reqwest, ReqwestMiddleware, ResponseError, Serde};
use synapse_client::models::{CommandMessage, QueryMessage};
use warp::http::StatusCode;
use warp::{Filter, Rejection};

pub async fn start_api() {
    let routes = issue_route()
        .or(redeem_route())
        .or(cancel_route())
        .or(query_one_route())
        .or(query_list_route());
    warp::serve(routes).run(([127, 0, 0, 1], 8090)).await;
}

fn issue_route() -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path!("issue" / "id" / String / "amount" / u32))
        .and_then(|id: String, amount: u32| async move {
            let command = IssueGiftCard {
                id: id.clone(),
                amount,
            };
            let command_message = to_command_message(IssueGiftCard::name(), Some(id), &command);
            send_and_handle_command_message(command_message).await
        })
}

fn redeem_route() -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path!("redeem" / "id" / String / "amount" / u32))
        .and_then(|id: String, amount: u32| async move {
            let command = RedeemGiftCard {
                id: id.clone(),
                amount,
            };
            let command_message = to_command_message(RedeemGiftCard::name(), Some(id), &command);
            send_and_handle_command_message(command_message).await
        })
}

fn cancel_route() -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path!("cancel" / "id" / String))
        .and_then(|id: String| async move {
            let command = CancelGiftCard { id: id.clone() };
            let command_message = to_command_message(CancelGiftCard::name(), Some(id), &command);
            send_and_handle_command_message(command_message).await
        })
}

fn query_one_route() -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::get()
        .and(warp::path!("query" / "id" / String))
        .and_then(|id: String| async move {
            let query = FetchGiftCardSummary { id };
            let query_message = to_query_message(FetchGiftCardSummary::name(), &query);
            send_and_handle_query_message(query_message).await
        })
}

fn query_list_route() -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::get()
        .and(warp::path!("query" / "limit" / u32 / "offset" / u32))
        .and_then(|limit: u32, offset: u32| async move {
            let query = FetchGiftCardSummaries { limit, offset };
            let query_message = to_query_message(FetchGiftCardSummaries::name(), &query);
            send_and_handle_query_message(query_message).await
        })
}

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    r#type: String,
    message: String,
}

async fn send_and_handle_command_message(
    message: CommandMessage,
) -> Result<warp::reply::WithStatus<warp::reply::Json>, Rejection> {
    let name = message.name.clone();
    match send_command_message(&CONFIGURATION, CONTEXT, Some(message)).await {
        Ok(r) => {
            let reply;
            if name == IssueGiftCard::name() {
                let message: IssueGiftCard = value_to_message(r.payload.unwrap().unwrap());
                reply = warp::reply::json(&message);
            } else if name == CancelGiftCard::name() {
                let message: CancelGiftCard = value_to_message(r.payload.unwrap().unwrap());
                reply = warp::reply::json(&message);
            } else if name == RedeemGiftCard::name() {
                let message: RedeemGiftCard = value_to_message(r.payload.unwrap().unwrap());
                reply = warp::reply::json(&message);
            } else {
                log::warn!("Unknown name for command {}, returning raw response.", name);
                reply = warp::reply::json(&r);
            }
            Ok::<warp::reply::WithStatus<warp::reply::Json>, Rejection>(warp::reply::with_status(
                reply,
                StatusCode::OK,
            ))
        }
        Err(e) => {
            let error_message = to_error_message(e);
            let rep = warp::reply::json(&error_message);
            Ok::<warp::reply::WithStatus<warp::reply::Json>, Rejection>(warp::reply::with_status(
                rep,
                StatusCode::from_u16(error_message.code).unwrap(),
            ))
        }
    }
}

async fn send_and_handle_query_message(
    message: QueryMessage,
) -> Result<warp::reply::WithStatus<warp::reply::Json>, Rejection> {
    let name = message.name.clone();
    match query_message(&CONFIGURATION, CONTEXT, Some(message)).await {
        Ok(r) => {
            let reply;
            if name == FetchGiftCardSummary::name() {
                let message: OneGiftCard = value_to_message(r.payload.unwrap().unwrap());
                reply = warp::reply::json(&message);
            } else if name == FetchGiftCardSummaries::name() {
                let message: MultipleGiftCards = value_to_message(r.payload.unwrap().unwrap());
                reply = warp::reply::json(&message);
            } else {
                log::warn!("Unknown name for query {}, returning raw response.", name);
                reply = warp::reply::json(&r);
            }
            Ok::<warp::reply::WithStatus<warp::reply::Json>, Rejection>(warp::reply::with_status(
                reply,
                StatusCode::OK,
            ))
        }
        Err(e) => {
            let error_message = to_error_message(e);
            let rep = warp::reply::json(&error_message);
            Ok::<warp::reply::WithStatus<warp::reply::Json>, Rejection>(warp::reply::with_status(
                rep,
                StatusCode::BAD_REQUEST,
            ))
        }
    }
}

fn to_error_message<T>(error: synapse_client::apis::Error<T>) -> ErrorMessage {
    match error {
        Reqwest(e) => {
            let code = match e.status() {
                None => 500,
                Some(code) => code.as_u16(),
            };
            ErrorMessage {
                code,
                r#type: "reqwest".to_string(),
                message: e.to_string(),
            }
        }
        ReqwestMiddleware(e) => {
            let code = match e.status() {
                None => 500,
                Some(code) => code.as_u16(),
            };
            ErrorMessage {
                code,
                r#type: "reqwest middle ware".to_string(),
                message: e.to_string(),
            }
        }
        Serde(e) => ErrorMessage {
            code: 500,
            r#type: "serde".to_string(),
            message: e.to_string(),
        },
        Io(e) => ErrorMessage {
            code: 500,
            r#type: "io".to_string(),
            message: e.to_string(),
        },
        ResponseError(e) => ErrorMessage {
            code: e.status.as_u16(),
            r#type: "response".to_string(),
            message: e.content,
        },
    }
}
