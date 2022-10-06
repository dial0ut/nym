export const handleCalculatePeriodRewards = ({
  estimatedOperatorReward,
  estimatedDelegatorsReward,
  totalDelegation,
  bondAmount,
}: {
  estimatedOperatorReward: number;
  estimatedDelegatorsReward: number;
  totalDelegation: string;
  bondAmount: string;
}) => {
  const dailyOperatorReward = (estimatedOperatorReward / 1_000_000) * 24; // epoch_reward * 1 epoch_per_hour * 24 hours
  const dailyDelegatorReward = (estimatedDelegatorsReward / 1_000_000) * 24;
  const operatorRewardScaled = 1000 * (dailyOperatorReward / +bondAmount);
  const delegatorRewardScaled = 1000 * (dailyDelegatorReward / +totalDelegation);
  const dailyTotal = operatorRewardScaled + delegatorRewardScaled;

  return {
    total: {
      daily: dailyTotal.toFixed(3).toString(),
      monthly: (dailyTotal * 30).toFixed(3).toString(),
      yearly: (dailyTotal * 365).toFixed(3).toString(),
    },
    operator: {
      daily: operatorRewardScaled.toFixed(3).toString(),
      monthly: (operatorRewardScaled * 30).toFixed(3).toString(),
      yearly: (operatorRewardScaled * 365).toFixed(3).toString(),
    },
    delegator: {
      daily: delegatorRewardScaled.toFixed(3).toString(),
      monthly: (delegatorRewardScaled * 30).toFixed(3).toString(),
      yearly: (delegatorRewardScaled * 365).toFixed(3).toString(),
    },
  };
};
