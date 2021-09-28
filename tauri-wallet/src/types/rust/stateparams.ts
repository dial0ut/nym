export interface TauriStateParams {
  epoch_length: number;
  minimum_mixnode_bond: string;
  minimum_gateway_bond: string;
  mixnode_bond_reward_rate: string;
  gateway_bond_reward_rate: string;
  mixnode_delegation_reward_rate: string;
  gateway_delegation_reward_rate: string;
  mixnode_active_set_size: number;
  gateway_active_set_size: number;
}