// Copyright 2022 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

use crate::NodeId;
use cosmwasm_std::{Addr, Coin, Decimal};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum MixnetContractError {
    #[error("{source}")]
    StdErr {
        #[from]
        source: cosmwasm_std::StdError,
    },

    #[error("Provided percent value is greater than 100%")]
    InvalidPercent,

    #[error("Attempted to subtract decimals with overflow ({minuend}.sub({subtrahend}))")]
    OverflowDecimalSubtraction {
        minuend: Decimal,
        subtrahend: Decimal,
    },

    #[error("Attempted to subtract with overflow ({minuend}.sub({subtrahend}))")]
    OverflowSubtraction { minuend: u64, subtrahend: u64 },

    #[error("Not enough funds sent for node pledge. (received {received}, minimum {minimum})")]
    InsufficientPledge { received: Coin, minimum: Coin },

    #[error("Not enough funds sent for node delegation. (received {received}, minimum {minimum})")]
    InsufficientDelegation { received: Coin, minimum: Coin },

    #[error("Mixnode ({id}) does not exist")]
    MixNodeBondNotFound { id: NodeId },

    #[error("{owner} does not seem to own any mixnodes")]
    NoAssociatedMixNodeBond { owner: Addr },

    #[error("{owner} does not seem to own any gateways")]
    NoAssociatedGatewayBond { owner: Addr },

    #[error("This address has already bonded a mixnode")]
    AlreadyOwnsMixnode,

    #[error("This address has already bonded a gateway")]
    AlreadyOwnsGateway,

    #[error("Gateway with this identity already exists. Its owner is {owner}")]
    DuplicateGateway { owner: Addr },

    #[error("Unauthorized")]
    Unauthorized,

    #[error("No tokens were sent for the bonding")]
    NoBondFound,

    #[error("No funds were provided for the delegation")]
    EmptyDelegation,

    #[error("Wrong coin denomination. Received: {received}, expected: {expected}")]
    WrongDenom { received: String, expected: String },

    #[error("Received multiple coin types during staking")]
    MultipleDenoms,

    #[error("Proxy address mismatch, expected {existing}, got {incoming}")]
    ProxyMismatch { existing: String, incoming: String },

    #[error("Failed to recover ed25519 public key from its base58 representation - {0}")]
    MalformedEd25519IdentityKey(String),

    #[error("Failed to recover ed25519 signature from its base58 representation - {0}")]
    MalformedEd25519Signature(String),

    #[error("Provided ed25519 signature did not verify correctly")]
    InvalidEd25519Signature,

    #[error("Can't perform the specified action as the current epoch is still progress. It started at {epoch_start} and finishes at {epoch_end}, while the current block time is {current_block_time}")]
    EpochInProgress {
        current_block_time: u64,
        epoch_start: i64,
        epoch_end: i64,
    },

    #[error("Mixnode {node_id} has already been rewarded during the current rewarding epoch ({absolute_epoch_id})")]
    MixnodeAlreadyRewarded {
        node_id: NodeId,
        absolute_epoch_id: u32,
    },

    #[error("Mixnode {node_id} hasn't been selected to the rewarding set in this epoch ({absolute_epoch_id})")]
    MixnodeNotInRewardedSet {
        node_id: NodeId,
        absolute_epoch_id: u32,
    },

    #[error("Mixnode {node_id} is currently in the process of unbonding")]
    MixnodeIsUnbonding { node_id: NodeId },

    #[error("Mixnode {node_id} has already unbonded")]
    MixnodeHasUnbonded { node_id: NodeId },

    #[error("The contract has ended up in a state that was deemed impossible: {comment}")]
    InconsistentState { comment: String },

    #[error(
        "Could not find any delegation information associated with mixnode {mix_id} for {address} (proxy: {proxy:?})"
    )]
    NoMixnodeDelegationFound {
        mix_id: NodeId,
        address: String,
        proxy: Option<String>,
    },

    #[error("Provided message to update rewarding params did not contain any updates")]
    EmptyParamsChangeMsg,

    #[error("Provided active set size is bigger than the rewarded set")]
    InvalidActiveSetSize,

    #[error("Provided rewarded set size is smaller than the active set")]
    InvalidRewardedSetSize,

    #[error("Provided active set size is zero")]
    ZeroActiveSet,

    #[error("Provided rewarded set size is zero")]
    ZeroRewardedSet,

    #[error("Received unexpected value for the active set. Got: {received}, expected: {expected}")]
    UnexpectedActiveSetSize { received: u32, expected: u32 },

    #[error("Received unexpected value for the rewarded set. Got: {received}, expected at most: {expected}")]
    UnexpectedRewardedSetSize { received: u32, expected: u32 },

    #[error("Mixnode {node_id} appears multiple times in the provided rewarded set update!")]
    DuplicateRewardedSetNode { node_id: NodeId },
}
