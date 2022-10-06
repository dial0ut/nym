import React, { useState, useEffect } from 'react';
import { Box, Card, CardContent, CardHeader, Grid, Typography } from '@mui/material';
import { decimalToPercentage } from '@nymproject/types';
import { ResultsTable } from 'src/components/RewardsPlayground/ResultsTable';
import { computeMixnodeRewardEstimation, getCurrentRewardingParameter, getDelegationSummary } from 'src/requests';
import { NodeDetails } from 'src/components/RewardsPlayground/NodeDetail';
import { Inputs, CalculateArgs } from 'src/components/RewardsPlayground/Inputs';
import { TBondedMixnode, useBondingContext } from 'src/context';
import { isMixnode } from 'src/types';
import { handleCalculatePeriodRewards } from './utils';

export type DefaultInputValues = {
  profitMargin: string;
  uptime: string;
  bond: string;
  delegations: string;
  operatorCost: string;
};

export const ApyPlayground = () => {
  const { bondedNode } = useBondingContext();

  const [results, setResults] = useState({
    total: { daily: '-', monthly: '-', yearly: '-' },
    operator: { daily: '-', monthly: '-', yearly: '-' },
    delegator: { daily: '-', monthly: '-', yearly: '-' },
  });

  const [defaultInputValues, setDefaultInputValues] = useState<DefaultInputValues>();
  const [stakeSaturation, setStakeSaturation] = useState<string>();

  const initialise = async (node: TBondedMixnode) => {
    const delegations = await getDelegationSummary();

    const res = await computeMixnodeRewardEstimation({
      mixId: node.id,
      performance: (node.uptime / 100).toString(),
      isActive: true,
      pledgeAmount: Math.floor(+node.bond.amount * 1_000_000),
      totalDelegation: Math.floor(+delegations.total_delegations.amount * 1_000_000),
    });

    setResults(
      handleCalculatePeriodRewards({
        estimatedOperatorReward: res.estimation.operator,
        estimatedDelegatorsReward: res.estimation.delegates,
        totalDelegation: delegations.total_delegations.amount,
        bondAmount: node.bond.amount,
      }),
    );
    setStakeSaturation(node.stakeSaturation);
    setDefaultInputValues({
      profitMargin: node.profitMargin,
      uptime: (node.uptime || 0).toString(),
      bond: node.bond.amount || '',
      delegations: delegations.total_delegations.amount,
      operatorCost: (res.estimation.operating_cost / 1_000_000).toFixed(3).toString(),
    });
  };

  useEffect(() => {
    if (bondedNode && isMixnode(bondedNode)) {
      initialise(bondedNode);
    }
  }, []);

  const handleCalculateEstimate = async ({ bond, delegations, uptime }: CalculateArgs) => {
    try {
      const res = await computeMixnodeRewardEstimation({
        mixId: bondedNode!.id,
        performance: (parseInt(uptime, 10) / 100).toString(),
        isActive: true,
        pledgeAmount: Math.floor(+bond * 1_000_000),
        totalDelegation: Math.floor(+delegations * 1_000_000),
      });
      const rewardParams = await getCurrentRewardingParameter();

      const computedStakeSaturation =
        (+bond * 1_000_000 + +delegations * 1_000_000) / Math.round(+rewardParams.interval.stake_saturation_point);

      const estimationResult = handleCalculatePeriodRewards({
        estimatedOperatorReward: res.estimation.operator,
        estimatedDelegatorsReward: res.estimation.delegates,
        totalDelegation: delegations,
        bondAmount: bond,
      });

      setStakeSaturation(decimalToPercentage(computedStakeSaturation.toString()));

      setResults(estimationResult);
    } catch (e) {
      console.log(e);
    }
  };

  return (
    <Box sx={{ p: 3 }}>
      <Typography fontWeight="medium" sx={{ mb: 1 }}>
        Playground
      </Typography>
      <Typography variant="body2" sx={{ color: 'grey.600', mb: 2 }}>
        This is your parameters playground - change the parameters below to see the node specific estimations in the
        table
      </Typography>
      <Card variant="outlined" sx={{ p: 1, mb: 3 }}>
        <CardHeader
          title={
            <Typography variant="body2" fontWeight="medium">
              Estimation calculator
            </Typography>
          }
        />
        <CardContent>
          {defaultInputValues && <Inputs onCalculate={handleCalculateEstimate} defaultValues={defaultInputValues} />}
        </CardContent>
      </Card>
      <Grid container spacing={3}>
        <Grid item xs={12} md={8}>
          <ResultsTable results={results} />
        </Grid>
        <Grid item xs={12} md={4}>
          <NodeDetails saturation={stakeSaturation} />
        </Grid>
      </Grid>
    </Box>
  );
};
