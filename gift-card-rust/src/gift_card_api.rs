use serde_derive::{Deserialize, Serialize};
use strum_macros::EnumIter;

// ########################################
// ############### Commands ###############
// ########################################

#[derive(Serialize, Deserialize, Debug, EnumIter)]
#[serde(tag = "type")]
pub enum GiftCardCommand {
    Issue(IssueGiftCard),
    Redeem(RedeemGiftCard),
    Cancel(CancelGiftCard),
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct IssueGiftCard {
    pub id: String,
    pub amount: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RedeemGiftCard {
    pub id: String,
    pub amount: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CancelGiftCard {
    pub id: String,
}

impl GiftCardCommand {
    pub fn id(&self) -> String {
        match self {
            GiftCardCommand::Issue(c) => c.id.to_owned().to_string(),
            GiftCardCommand::Redeem(c) => c.id.to_owned().to_string(),
            GiftCardCommand::Cancel(c) => c.id.to_owned().to_string(),
        }
    }
}

// ########################################
// ################ Events ################
// ########################################

#[derive(Serialize, Deserialize, Debug, Clone, EnumIter)]
#[serde(tag = "type")]
pub enum GiftCardEvent {
    Issue(GiftCardIssued),
    Redeem(GiftCardRedeemed),
    Cancel(GiftCardCanceled),
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GiftCardIssued {
    pub id: String,
    pub amount: u32,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GiftCardRedeemed {
    pub id: String,
    pub amount: u32,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GiftCardCanceled {
    pub id: String,
}

impl GiftCardEvent {
    pub fn id(&self) -> String {
        match self {
            GiftCardEvent::Issue(c) => c.id.to_owned().to_string(),
            GiftCardEvent::Redeem(c) => c.id.to_owned().to_string(),
            GiftCardEvent::Cancel(c) => c.id.to_owned().to_string(),
        }
    }
}

impl GiftCardCommand {
    pub fn payload_type(&self) -> String {
        match self {
            GiftCardCommand::Issue(_c) => "IssueGiftCard".to_string(),
            GiftCardCommand::Redeem(_c) => "RedeemGiftCard".to_string(),
            GiftCardCommand::Cancel(_c) => "CancelGiftCard".to_string(),
        }
    }
}

impl GiftCardEvent {
    pub fn payload_type(&self) -> String {
        match self {
            GiftCardEvent::Issue(_c) => "GiftCardIssued".to_string(),
            GiftCardEvent::Redeem(_c) => "GiftCardRedeemed".to_string(),
            GiftCardEvent::Cancel(_c) => "GiftCardCanceled".to_string(),
        }
    }
}

impl GiftCardEvent {
    pub fn aggregate_type(&self) -> String {
        "GiftCard".to_string()
    }
}

// ########################################
// ############### Queries ###############
// ########################################

#[derive(Serialize, Deserialize, Debug, EnumIter)]
#[serde(tag = "type")]
pub enum GiftCardQuery {
    ById(FindGiftCardById),
    All(FindAllGiftCards),
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FindGiftCardById {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FindAllGiftCards;

impl GiftCardQuery {
    pub fn payload_type(&self) -> String {
        match self {
            GiftCardQuery::ById(_q) => "FindGiftCardById".to_string(),
            GiftCardQuery::All(_q) => "FindAllGiftCards".to_string(),
        }
    }
}
