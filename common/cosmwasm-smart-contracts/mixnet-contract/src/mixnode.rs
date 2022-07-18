// due to code generated by JsonSchema
#![allow(clippy::field_reassign_with_default)]

use crate::constants::UNIT_DELEGATION_BASE;
use crate::error::MixnetContractError;
use crate::interval::FullEpochId;
use crate::reward_params::{NodeRewardParams, RewardingParams};
use crate::rewarding::helpers::truncate_reward;
use crate::rewarding::RewardDistribution;
use crate::{Delegation, IdentityKey, NodeId, Percent, SphinxKey};
use cosmwasm_std::{coin, Addr, Coin, Decimal, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::Display;

#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/RewardedSetNodeStatus.ts")
)]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq)]
pub enum RewardedSetNodeStatus {
    Active,
    Standby,
}

impl RewardedSetNodeStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, RewardedSetNodeStatus::Active)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct MixNodeDetails {
    pub bond_information: MixNodeBond,

    pub rewarding_details: MixNodeRewarding,
}

impl MixNodeDetails {
    pub fn new(bond_information: MixNodeBond, rewarding_details: MixNodeRewarding) -> Self {
        MixNodeDetails {
            bond_information,
            rewarding_details,
        }
    }

    pub fn mix_id(&self) -> NodeId {
        self.bond_information.id
    }

    pub fn is_unbonding(&self) -> bool {
        self.bond_information.is_unbonding
    }

    pub fn original_pledge(&self) -> &Coin {
        &self.bond_information.original_pledge
    }

    pub fn pending_operator_reward(&self) -> Coin {
        let pledge = self.original_pledge();
        self.rewarding_details.pending_operator_reward(pledge)
    }
}

// the fields on this one are not really finalised yet and I don't think they're going to be until
// I properly implement the thing
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct MixNodeRewarding {
    /// Information provided by the operator that influence the cost function.    
    #[serde(rename = "cp")]
    pub cost_params: MixNodeCostParams,

    /// Total pledge and compounded reward earned by the node operator.
    #[serde(rename = "op")]
    pub operator: Decimal,

    /// Total delegation and compounded reward earned by all node delegators.
    #[serde(rename = "dg")]
    pub delegates: Decimal,

    /// Cumulative reward earned by the "unit delegation" since the block 0.
    #[serde(rename = "tur")]
    pub total_unit_reward: Decimal,

    /// Value of the theoretical "unit delegation" that has delegated to this mixnode at block 0.
    #[serde(rename = "ud")]
    pub unit_delegation: Decimal,

    // // TODO: this might be possibly redundant
    // /// Rewards accumulated by the "unit delegation" in the current period.
    // pub current_period_reward: Decimal,
    /// Marks the epoch when this node was last rewarded so that we wouldn't accidentally attempt
    /// to reward it multiple times in the same epoch.
    #[serde(rename = "le")]
    pub last_rewarded_epoch: FullEpochId,

    // technically we don't need that field to determine reward magnitude or anything
    // but it saves on extra queries to determine if we're removing the final delegation
    // (so that we could zero the field correctly)
    #[serde(rename = "uqd")]
    pub unique_delegations: u32,
}

impl MixNodeRewarding {
    pub fn initialise_new(
        cost_params: MixNodeCostParams,
        initial_pledge: &Coin,
        current_epoch: FullEpochId,
    ) -> Self {
        MixNodeRewarding {
            cost_params,
            operator: Decimal::from_atomics(initial_pledge.amount, 0).unwrap(),
            delegates: Decimal::zero(),
            total_unit_reward: Decimal::zero(),
            unit_delegation: UNIT_DELEGATION_BASE,
            last_rewarded_epoch: current_epoch,
            unique_delegations: 0,
        }
    }

    /// Determines whether this node is still bonded. This is performed via a simple check,
    /// if there are no tokens left associated with the operator, it means they have unbonded
    /// and those params only exist for the purposes of calculating rewards for delegators that
    /// have not yet removed their tokens.
    pub fn still_bonded(&self) -> bool {
        self.operator != Decimal::zero()
    }

    pub fn pending_operator_reward(&self, original_pledge: &Coin) -> Coin {
        let reward_with_pledge = truncate_reward(self.operator, &original_pledge.denom);
        Coin {
            denom: reward_with_pledge.denom,
            amount: reward_with_pledge.amount - original_pledge.amount,
        }
    }

    pub fn operator_pledge_with_reward(&self, denom: impl Into<String>) -> Coin {
        truncate_reward(self.operator, denom)
    }

    pub fn pending_delegator_reward(&self, delegation: &Delegation) -> Coin {
        let delegator_reward = self.determine_delegation_reward(delegation);
        truncate_reward(delegator_reward, &delegation.amount.denom)
    }

    pub fn withdraw_operator_reward(&mut self, original_pledge: &Coin) -> Coin {
        let initial_dec = Decimal::from_atomics(original_pledge.amount, 0).unwrap();
        if initial_dec > self.operator {
            panic!(
                "seems slashing has occurred while it has not been implemented nor accounted for!"
            )
        }
        let diff = self.operator - initial_dec;
        self.operator = initial_dec;

        truncate_reward(diff, &original_pledge.denom)
    }

    pub fn withdraw_delegator_reward(
        &mut self,
        delegation: &mut Delegation,
    ) -> Result<Coin, MixnetContractError> {
        let reward = self.determine_delegation_reward(delegation);
        self.decrease_delegates(reward)?;

        delegation.cumulative_reward_ratio = self.full_reward_ratio();
        Ok(truncate_reward(reward, &delegation.amount.denom))
    }

    pub fn node_bond(&self) -> Decimal {
        self.operator + self.delegates
    }

    /// Saturation over the tokens pledged by the node operator.
    pub fn pledge_saturation(&self, reward_params: &RewardingParams) -> Decimal {
        // make sure our saturation is never greater than 1
        if self.operator > reward_params.interval.stake_saturation_point {
            Decimal::one()
        } else {
            self.operator / reward_params.interval.stake_saturation_point
        }
    }

    /// Saturation over all the tokens staked over this node.
    pub fn bond_saturation(&self, reward_params: &RewardingParams) -> Decimal {
        // make sure our saturation is never greater than 1
        if self.node_bond() > reward_params.interval.stake_saturation_point {
            Decimal::one()
        } else {
            self.node_bond() / reward_params.interval.stake_saturation_point
        }
    }

    pub fn uncapped_bond_saturation(&self, reward_params: &RewardingParams) -> Decimal {
        self.node_bond() / reward_params.interval.stake_saturation_point
    }

    pub fn node_reward(
        &self,
        reward_params: &RewardingParams,
        node_params: NodeRewardParams,
    ) -> Decimal {
        let work = if node_params.in_active_set {
            reward_params.active_node_work()
        } else {
            reward_params.standby_node_work()
        };

        let alpha = reward_params.interval.sybil_resistance;

        reward_params.interval.epoch_reward_budget
            * node_params.performance.value()
            * self.bond_saturation(reward_params)
            * (work
                + alpha.value() * self.pledge_saturation(reward_params)
                    / reward_params.dec_rewarded_set_size())
            / (Decimal::one() + alpha.value())
    }

    pub fn determine_reward_split(
        &self,
        node_reward: Decimal,
        node_performance: Percent,
        // I don't like this argument here, makes things look, idk, messy...
        epochs_in_interval: u32,
    ) -> RewardDistribution {
        let node_cost =
            self.cost_params.epoch_operating_cost(epochs_in_interval) * node_performance.value();

        // check if profit is positive
        if node_reward > node_cost {
            let profit = node_reward - node_cost;
            let profit_margin = self.cost_params.profit_margin_percent.value();
            let one = Decimal::one();

            let operator_share = self.operator / self.node_bond();

            let operator = profit * (profit_margin + (one - profit_margin) * operator_share);
            let delegates = profit - operator;

            debug_assert_eq!(operator + delegates + node_cost, node_reward);

            RewardDistribution {
                operator: operator + node_cost,
                delegates,
            }
        } else {
            RewardDistribution {
                operator: node_reward,
                delegates: Decimal::zero(),
            }
        }
    }

    pub fn calculate_epoch_reward(
        &self,
        reward_params: &RewardingParams,
        node_params: NodeRewardParams,
        epochs_in_interval: u32,
    ) -> RewardDistribution {
        let node_reward = self.node_reward(reward_params, node_params);
        self.determine_reward_split(node_reward, node_params.performance, epochs_in_interval)
    }

    pub fn distribute_rewards(
        &mut self,
        distribution: RewardDistribution,
        full_epoch_id: FullEpochId,
    ) {
        let unit_delegation_reward = distribution.delegates
            * self.delegator_share(self.unit_delegation + self.total_unit_reward);

        self.operator += distribution.operator;
        self.delegates += distribution.delegates;

        // self.current_period_reward += unit_delegation_reward;
        self.total_unit_reward += unit_delegation_reward;
        self.last_rewarded_epoch = full_epoch_id;
    }

    pub fn epoch_rewarding(
        &mut self,
        reward_params: &RewardingParams,
        node_params: NodeRewardParams,
        epochs_in_interval: u32,
        full_epoch_id: FullEpochId,
    ) {
        let reward_distribution =
            self.calculate_epoch_reward(reward_params, node_params, epochs_in_interval);
        self.distribute_rewards(reward_distribution, full_epoch_id)
    }

    // pub fn increment_period(&mut self) -> HistoricalRewards {
    //     // let rewards = self.current_period_reward;
    //
    //     // self.past_periods_sum += rewards;
    //     // self.current_period_reward = Decimal::zero();
    //     self.current_period += 1;
    //
    //     // note: this already includes the sum for the period that just finished
    //     HistoricalRewards::new(self.total_unit_reward)
    // }

    pub fn determine_delegation_reward(&self, delegation: &Delegation) -> Decimal {
        let starting_ratio = delegation.cumulative_reward_ratio;
        let ending_ratio = self.full_reward_ratio();
        let adjust = starting_ratio + UNIT_DELEGATION_BASE;

        (ending_ratio - starting_ratio) * delegation.dec_amount() / adjust
    }

    // Special care must be taken when calling this method as it is expected it's called in conjunction
    // with `increment_period`
    pub fn add_base_delegation(&mut self, amount: Uint128) {
        // the unwrap here is fine as the value is guaranteed to fit under provided constraints
        self.delegates += Decimal::from_atomics(amount, 0).unwrap()
    }

    pub fn decrease_delegates(&mut self, amount: Decimal) -> Result<(), MixnetContractError> {
        if self.delegates < amount {
            return Err(MixnetContractError::OverflowDecimalSubtraction {
                minuend: self.delegates,
                subtrahend: amount,
            });
        }

        self.delegates -= amount;
        Ok(())
    }

    pub fn decrease_operator(&mut self, amount: Decimal) -> Result<(), MixnetContractError> {
        if self.operator < amount {
            return Err(MixnetContractError::OverflowDecimalSubtraction {
                minuend: self.operator,
                subtrahend: amount,
            });
        }

        self.operator -= amount;
        Ok(())
    }

    pub fn full_reward_ratio(&self) -> Decimal {
        self.total_unit_reward //+ self.current_period_reward
    }

    pub fn delegator_share(&self, amount: Decimal) -> Decimal {
        if self.delegates.is_zero() {
            Decimal::zero()
        } else {
            amount / self.delegates
        }
    }
}

// operator information + data assigned by the contract(s)
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct MixNodeBond {
    /// Unique id assigned to the bonded mixnode.
    pub id: NodeId,

    /// Address of the owner of this mixnode.
    pub owner: Addr,

    // TODO: do we even care about this field?
    /// Original amount pledged by the operator of this node.
    pub original_pledge: Coin,

    /// Layer assigned to this mixnode.
    pub layer: Layer,

    /// Information provided by the operator for the purposes of bonding.
    pub mix_node: MixNode,

    /// Entity who bonded this mixnode on behalf of the owner.
    /// If exists, it's most likely the address of the vesting contract.
    pub proxy: Option<Addr>,

    /// Block height at which this mixnode has been bonded.
    pub bonding_height: u64,

    /// Flag to indicate whether this node is in the process of unbonding,
    /// that will conclude upon the epoch finishing.
    pub is_unbonding: bool,
}

impl MixNodeBond {
    pub fn new(
        id: NodeId,
        owner: Addr,
        original_pledge: Coin,
        layer: Layer,
        mix_node: MixNode,
        proxy: Option<Addr>,
        bonding_height: u64,
    ) -> Self {
        MixNodeBond {
            id,
            owner,
            original_pledge,
            layer,
            mix_node,
            proxy,
            bonding_height,
            is_unbonding: false,
        }
    }

    pub fn identity(&self) -> &str {
        &self.mix_node.identity_key
    }

    pub fn original_pledge(&self) -> &Coin {
        &self.original_pledge
    }

    pub fn owner(&self) -> &Addr {
        &self.owner
    }

    pub fn mix_node(&self) -> &MixNode {
        &self.mix_node
    }
}

// information provided by the operator
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct MixNode {
    /// Network address of this mixnode, for example 1.1.1.1:1234 or foo.mixnode.com
    pub host: String,

    pub mix_port: u16,

    pub verloc_port: u16,

    pub http_api_port: u16,

    /// Base58-encoded x25519 public key used for sphinx key derivation.
    pub sphinx_key: SphinxKey,

    /// Base58-encoded ed25519 EdDSA public key.
    pub identity_key: IdentityKey,

    pub version: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct MixNodeCostParams {
    pub profit_margin_percent: Percent,

    /// Operating cost of the associated mixnode per the entire interval.
    pub interval_operating_cost: Coin,
}

impl MixNodeCostParams {
    pub fn to_inline_json(&self) -> String {
        // as per documentation on `to_string`:
        //      > Serialization can fail if `T`'s implementation of `Serialize` decides to
        //      > fail, or if `T` contains a map with non-string keys.
        // We have derived the `Serialize`, thus we're pretty confident it's valid and
        // the struct does not contain any maps, so the unwrap here is fine.
        serde_json::to_string(self).unwrap()
    }
}

impl MixNodeCostParams {
    pub fn epoch_operating_cost(&self, epochs_in_interval: u32) -> Decimal {
        Decimal::from_ratio(self.interval_operating_cost.amount, epochs_in_interval)
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize_repr,
    Deserialize_repr,
    JsonSchema,
)]
#[repr(u8)]
pub enum Layer {
    One = 1,
    Two = 2,
    Three = 3,
}

impl From<Layer> for String {
    fn from(layer: Layer) -> Self {
        (layer as u8).to_string()
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct UnbondedMixnode {
    pub identity: IdentityKey,
    pub owner: Addr,
    pub unbonding_height: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct MixNodeConfigUpdate {
    pub host: String,
    pub mix_port: u16,
    pub verloc_port: u16,
    pub http_api_port: u16,
    pub version: String,
}

impl MixNodeConfigUpdate {
    pub fn to_inline_json(&self) -> String {
        // as per documentation on `to_string`:
        //      > Serialization can fail if `T`'s implementation of `Serialize` decides to
        //      > fail, or if `T` contains a map with non-string keys.
        // We have derived the `Serialize`, thus we're pretty confident it's valid and
        // the struct does not contain any maps, so the unwrap here is fine.
        serde_json::to_string(self).unwrap()
    }
}

//
// impl PartialOrd for MixNodeBond {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         // first remove invalid cases
//         if self.pledge_amount.denom != self.total_delegation.denom {
//             return None;
//         }
//
//         if other.pledge_amount.denom != other.total_delegation.denom {
//             return None;
//         }
//
//         if self.pledge_amount.denom != other.pledge_amount.denom {
//             return None;
//         }
//
//         // try to order by total bond + delegation
//         let total_cmp = (self.pledge_amount.amount + self.total_delegation.amount)
//             .partial_cmp(&(self.pledge_amount.amount + self.total_delegation.amount))?;
//
//         if total_cmp != Ordering::Equal {
//             return Some(total_cmp);
//         }
//
//         // then if those are equal, prefer higher bond over delegation
//         let pledge_cmp = self
//             .pledge_amount
//             .amount
//             .partial_cmp(&other.pledge_amount.amount)?;
//         if pledge_cmp != Ordering::Equal {
//             return Some(pledge_cmp);
//         }
//
//         // then look at delegation (I'm not sure we can get here, but better safe than sorry)
//         let delegation_cmp = self
//             .total_delegation
//             .amount
//             .partial_cmp(&other.total_delegation.amount)?;
//         if delegation_cmp != Ordering::Equal {
//             return Some(delegation_cmp);
//         }
//
//         // then check block height
//         let height_cmp = self.bonding_height.partial_cmp(&other.bonding_height)?;
//         if height_cmp != Ordering::Equal {
//             return Some(height_cmp);
//         }
//
//         // finally go by the rest of the fields in order. It doesn't really matter at this point
//         // but we should be deterministic.
//         let owner_cmp = self.owner.partial_cmp(&other.owner)?;
//         if owner_cmp != Ordering::Equal {
//             return Some(owner_cmp);
//         }
//
//         let layer_cmp = self.layer.partial_cmp(&other.layer)?;
//         if layer_cmp != Ordering::Equal {
//             return Some(layer_cmp);
//         }
//
//         self.mix_node.partial_cmp(&other.mix_node)
//     }
// }

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct PagedMixnodeBondsResponse {
    pub nodes: Vec<MixNodeBond>,
    pub per_page: usize,
    pub start_next_after: Option<NodeId>,
}

impl PagedMixnodeBondsResponse {
    pub fn new(nodes: Vec<MixNodeBond>, per_page: usize, start_next_after: Option<NodeId>) -> Self {
        PagedMixnodeBondsResponse {
            nodes,
            per_page,
            start_next_after,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct PagedMixnodesDetailsResponse {
    pub nodes: Vec<MixNodeDetails>,
    pub per_page: usize,
    pub start_next_after: Option<NodeId>,
}

impl PagedMixnodesDetailsResponse {
    pub fn new(
        nodes: Vec<MixNodeDetails>,
        per_page: usize,
        start_next_after: Option<NodeId>,
    ) -> Self {
        PagedMixnodesDetailsResponse {
            nodes,
            per_page,
            start_next_after,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct PagedUnbondedMixnodesResponse {
    pub nodes: Vec<(NodeId, UnbondedMixnode)>,
    pub per_page: usize,
    pub start_next_after: Option<NodeId>,
}

impl PagedUnbondedMixnodesResponse {
    pub fn new(
        nodes: Vec<(NodeId, UnbondedMixnode)>,
        per_page: usize,
        start_next_after: Option<NodeId>,
    ) -> Self {
        PagedUnbondedMixnodesResponse {
            nodes,
            per_page,
            start_next_after,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct MixOwnershipResponse {
    pub address: Addr,
    pub mixnode_details: Option<MixNodeDetails>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct MixnodeDetailsResponse {
    pub mix_id: NodeId,
    pub mixnode_details: Option<MixNodeDetails>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct MixnodeRewardingDetailsResponse {
    pub mix_id: NodeId,
    pub rewarding_details: Option<MixNodeRewarding>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct UnbondedMixnodeResponse {
    pub mix_id: NodeId,
    pub unbonded_info: Option<UnbondedMixnode>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct StakeSaturationResponse {
    pub mix_id: NodeId,
    pub current_saturation: Option<Decimal>,
    pub uncapped_saturation: Option<Decimal>,
}

//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     fn mixnode_fixture() -> MixNode {
//         MixNode {
//             host: "1.1.1.1".to_string(),
//             mix_port: 123,
//             verloc_port: 456,
//             http_api_port: 789,
//             sphinx_key: "sphinxkey".to_string(),
//             identity_key: "identitykey".to_string(),
//             version: "0.11.0".to_string(),
//             profit_margin_percent: 10,
//         }
//     }
//
//     #[test]
//     fn mixnode_bond_partial_ord() {
//         let _150foos = Coin::new(150, "foo");
//         let _50foos = Coin::new(50, "foo");
//         let _0foos = Coin::new(0, "foo");
//
//         let mix1 = MixNodeBond {
//             pledge_amount: _150foos.clone(),
//             total_delegation: _50foos.clone(),
//             owner: Addr::unchecked("foo1"),
//             layer: Layer::One,
//             bonding_height: 100,
//             mix_node: mixnode_fixture(),
//             proxy: None,
//             accumulated_rewards: Some(Uint128::zero()),
//         };
//
//         let mix2 = MixNodeBond {
//             pledge_amount: _150foos.clone(),
//             total_delegation: _50foos.clone(),
//             owner: Addr::unchecked("foo2"),
//             layer: Layer::One,
//             bonding_height: 120,
//             mix_node: mixnode_fixture(),
//             proxy: None,
//             accumulated_rewards: Some(Uint128::zero()),
//         };
//
//         let mix3 = MixNodeBond {
//             pledge_amount: _50foos,
//             total_delegation: _150foos.clone(),
//             owner: Addr::unchecked("foo3"),
//             layer: Layer::One,
//             bonding_height: 120,
//             mix_node: mixnode_fixture(),
//             proxy: None,
//             accumulated_rewards: Some(Uint128::zero()),
//         };
//
//         let mix4 = MixNodeBond {
//             pledge_amount: _150foos.clone(),
//             total_delegation: _0foos.clone(),
//             owner: Addr::unchecked("foo4"),
//             layer: Layer::One,
//             bonding_height: 120,
//             mix_node: mixnode_fixture(),
//             proxy: None,
//             accumulated_rewards: Some(Uint128::zero()),
//         };
//
//         let mix5 = MixNodeBond {
//             pledge_amount: _0foos,
//             total_delegation: _150foos,
//             owner: Addr::unchecked("foo5"),
//             layer: Layer::One,
//             bonding_height: 120,
//             mix_node: mixnode_fixture(),
//             proxy: None,
//             accumulated_rewards: Some(Uint128::zero()),
//         };
//
//         // summary:
//         // mix1: 150bond + 50delegation, foo1, 100
//         // mix2: 150bond + 50delegation, foo2, 120
//         // mix3: 50bond + 150delegation, foo3, 120
//         // mix4: 150bond + 0delegation, foo4, 120
//         // mix5: 0bond + 150delegation, foo5, 120
//
//         // highest total bond+delegation is used
//         // then bond followed by delegation
//         // finally just the rest of the fields
//
//         // mix1 has higher total than mix4 or mix5
//         assert!(mix1 > mix4);
//         assert!(mix1 > mix5);
//
//         // mix1 has the same total as mix3, however, mix1 has more tokens in bond
//         assert!(mix1 > mix3);
//         // same case for mix4 and mix5
//         assert!(mix4 > mix5);
//
//         // same bond and delegation, so it's just ordered by height
//         assert!(mix1 < mix2);
//     }
// }
