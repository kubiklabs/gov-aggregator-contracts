use cosmwasm_std::StdError;
use cw_denom::DenomError;
use cw_utils::ParseReplyError;
use thiserror::Error;

use cwd_voting::{deposit::DepositError, status::Status};

#[derive(Error, Debug, PartialEq)]
pub enum PreProposeError {
    #[error(transparent)]
    Std(#[from] StdError),

    #[error(transparent)]
    Denom(#[from] DenomError),

    #[error(transparent)]
    Deposit(#[from] DepositError),

    #[error("Message sender is not proposal module")]
    NotModule {},

    #[error("Message sender is not dao")]
    NotDao {},

    #[error("Proposal is malformed")]
    MalformedProposal {},

    #[error("You must be a member of this DAO (have voting power) to create a proposal")]
    NotMember {},

    #[error("No denomination for withdrawal. specify a denomination to withdraw")]
    NoWithdrawalDenom {},

    #[error("Nothing to withdraw")]
    NothingToWithdraw {},

    #[error("Proposal status ({status}) not closed or executed")]
    NotClosedOrExecuted { status: Status },

    #[error("Multiple voting modules during instantiation.")]
    MultipleTimelockModules {},

    #[error("An unknown reply ID was received.")]
    UnknownReplyID {},

    #[error(transparent)]
    ParseReplyError(#[from] ParseReplyError),
}
