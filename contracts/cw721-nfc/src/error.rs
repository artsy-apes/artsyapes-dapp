use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.

    #[error("Contract is paused and can't be interacted with")]
    ContractIsPaused {},

    #[error("You already own this physical item")]
    AlreadyOwned {},

    #[error("You need to provide correct tier parameter")]
    InvalidTier {},

    #[error("Max number of Tier 1 Physical Items")]
    MaxTier1Items {},

    #[error("Max number of Tier 2 Physical Items")]
    MaxTier2Items {},

    #[error("Max number of Tier 3 Physical Items")]
    MaxTier3Items {},

    #[error("Invalid tokens sent")]
    InvalidUSTAmount {
        required: u128,
        sent: u128
    },

    #[error("Only UST among native tokens accepted")]
    OnlyUSTAccepted {},

    #[error("Tier max Limit can't be set to zero")]
    TierMaxLimitIsZero {},

    #[error("You need to over-bid previous bidder")]
    LowBidding {},

    #[error("Unauthorized")]
    BiddingNotAllowed {},
}
