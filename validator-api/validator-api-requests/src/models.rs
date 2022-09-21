// Copyright 2022 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

use cosmwasm_std::Decimal;
use mixnet_contract_common::mixnode::MixNodeDetails;
use mixnet_contract_common::reward_params::{Performance, RewardingParams};
use mixnet_contract_common::rewarding::RewardEstimate;
use mixnet_contract_common::{Interval, MixNode, NodeId, RewardedSetNodeStatus};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fmt, time::Duration};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/MixnodeStatus.ts")
)]
#[serde(rename_all = "snake_case")]
pub enum MixnodeStatus {
    Active,   // in both the active set and the rewarded set
    Standby,  // only in the rewarded set
    Inactive, // in neither the rewarded set nor the active set, but is bonded
    NotFound, // doesn't even exist in the bonded set
}

impl From<MixnodeStatus> for Option<RewardedSetNodeStatus> {
    fn from(status: MixnodeStatus) -> Self {
        match status {
            MixnodeStatus::Active => Some(RewardedSetNodeStatus::Active),
            MixnodeStatus::Standby => Some(RewardedSetNodeStatus::Standby),
            MixnodeStatus::Inactive => None,
            MixnodeStatus::NotFound => None,
        }
    }
}

impl MixnodeStatus {
    pub fn is_active(&self) -> bool {
        *self == MixnodeStatus::Active
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/MixnodeCoreStatusResponse.ts")
)]
pub struct MixnodeCoreStatusResponse {
    pub mix_id: NodeId,
    pub count: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/GatewayCoreStatusResponse.ts")
)]
pub struct GatewayCoreStatusResponse {
    pub identity: String,
    pub count: i32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/MixnodeStatusResponse.ts")
)]
pub struct MixnodeStatusResponse {
    pub status: MixnodeStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct MixNodeBondAnnotated {
    pub mixnode_details: MixNodeDetails,
    pub stake_saturation: StakeSaturation,
    pub uncapped_stake_saturation: StakeSaturation,
    pub performance: Performance,
    pub estimated_operator_apy: Decimal,
    pub estimated_delegators_apy: Decimal,
}

impl MixNodeBondAnnotated {
    pub fn mix_node(&self) -> &MixNode {
        &self.mixnode_details.bond_information.mix_node
    }

    pub fn mix_id(&self) -> NodeId {
        self.mixnode_details.mix_id()
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ComputeRewardEstParam {
    pub performance: Option<Performance>,
    pub active_in_rewarded_set: Option<bool>,
    pub pledge_amount: Option<u64>,
    pub total_delegation: Option<u64>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema)]
pub struct RewardEstimationResponse {
    pub estimation: RewardEstimate,

    pub reward_params: RewardingParams,
    pub epoch: Interval,
    pub as_at: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct UptimeResponse {
    pub mix_id: NodeId,
    pub avg_uptime: u8,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/StakeSaturationResponse.ts")
)]
pub struct StakeSaturationResponse {
    #[cfg_attr(feature = "generate-ts", ts(type = "string"))]
    pub saturation: StakeSaturation,

    #[cfg_attr(feature = "generate-ts", ts(type = "string"))]
    pub uncapped_saturation: StakeSaturation,
    pub as_at: i64,
}

pub type StakeSaturation = Decimal;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/SelectionChance.ts")
)]
pub enum SelectionChance {
    High,
    Good,
    Low,
}

impl From<f64> for SelectionChance {
    fn from(p: f64) -> SelectionChance {
        match p {
            p if p >= 0.7 => SelectionChance::High,
            p if p >= 0.3 => SelectionChance::Good,
            _ => SelectionChance::Low,
        }
    }
}

impl From<Decimal> for SelectionChance {
    fn from(p: Decimal) -> Self {
        match p {
            p if p >= Decimal::from_ratio(70u32, 100u32) => SelectionChance::High,
            p if p >= Decimal::from_ratio(30u32, 100u32) => SelectionChance::Good,
            _ => SelectionChance::Low,
        }
    }
}

impl fmt::Display for SelectionChance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SelectionChance::High => write!(f, "High"),
            SelectionChance::Good => write!(f, "Good"),
            SelectionChance::Low => write!(f, "Low"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/InclusionProbabilityResponse.ts")
)]
pub struct InclusionProbabilityResponse {
    pub in_active: SelectionChance,
    pub in_reserve: SelectionChance,
}

impl fmt::Display for InclusionProbabilityResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "in_active: {}, in_reserve: {}",
            self.in_active, self.in_reserve
        )
    }
}

#[derive(Clone, Serialize, schemars::JsonSchema)]
pub struct AllInclusionProbabilitiesResponse {
    pub inclusion_probabilities: Vec<InclusionProbability>,
    pub samples: u64,
    pub elapsed: Duration,
    pub delta_max: f64,
    pub delta_l2: f64,
    pub as_at: i64,
}

#[derive(Clone, Serialize, schemars::JsonSchema)]
pub struct InclusionProbability {
    pub id: NodeId,
    pub in_active: f64,
    pub in_reserve: f64,
}
