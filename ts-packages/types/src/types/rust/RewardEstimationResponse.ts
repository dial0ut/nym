export type RewardEstimationResponse = {
  estimation: {
    total_node_reward: number;
    operator: number;
    delegates: number;
    operating_cost: number;
  };
  reward_params: {
    rewarded_set_size: number;
    active_set_size: number;
  };
  as_at: number;
};
