use serde_derive::{Deserialize, Serialize};

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

#[derive(Debug)]
pub enum GiftCardCommand {
    Issue(IssueGiftCard),
    Redeem(RedeemGiftCard),
    Cancel(CancelGiftCard),
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


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GiftCardIssued {
    pub id: String,
    pub amount: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GiftCardRedeemed {
    pub id: String,
    pub amount: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GiftCardCanceled {
    pub id: String,
}

#[derive(Debug, Clone)]
pub enum GiftCardEvent {
    Issue(GiftCardIssued),
    Redeem(GiftCardRedeemed),
    Cancel(GiftCardCanceled),
}