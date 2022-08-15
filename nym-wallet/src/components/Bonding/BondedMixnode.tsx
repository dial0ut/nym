import React from 'react';
import { Box, Button, Stack, Tooltip, Typography } from '@mui/material';
import { Link } from '@nymproject/react/link/Link';
import { TBondedMixnode, urls } from 'src/context';
import { NymCard } from 'src/components';
import { Network } from 'src/types';
import { IdentityKey } from 'src/components/IdentityKey';
import { NodeStatus } from 'src/components/NodeStatus';
import { Node as NodeIcon } from '../../svg-icons/node';
import { Cell, Header, NodeTable } from './NodeTable';
import { BondedMixnodeActions, TBondedMixnodeActions } from './BondedMixnodeActions';
import { NodeStats } from './NodeStats';

const headers: Header[] = [
  {
    header: 'Stake',
    id: 'stake',
    sx: { pl: 0 },
  },
  {
    header: 'Bond',
    id: 'bond',
  },
  {
    header: 'Stake saturation',
    id: 'stake-saturation',
  },
  {
    header: 'PM',
    id: 'profit-margin',
    tooltipText:
      'The percentage of the node rewards that you as the node operator will take before the rest of the reward is shared between you and the delegators.',
  },
  {
    header: 'Operator cost',
    id: 'operator-cost',
    // tooltipText: 'TODO', // TODO
  },
  {
    header: 'Operator rewards',
    id: 'operator-rewards',
    tooltipText:
      'This is your (operator) new rewards including the PM and cost. You can compound your rewards manually every epoch or unbond your node to redeem them.',
  },
  {
    header: 'No. delegators',
    id: 'delegators',
  },
  {
    id: 'menu-button',
    sx: { width: 34, maxWidth: 34 },
  },
];

export const BondedMixnode = ({
  mixnode,
  network,
  onActionSelect,
}: {
  mixnode: TBondedMixnode;
  network?: Network;
  onActionSelect: (action: TBondedMixnodeActions) => void;
}) => {
  const {
    name,
    stake,
    bond,
    stakeSaturation,
    profitMargin,
    operatorRewards,
    operatorCost,
    delegators,
    status,
    identityKey,
    host,
  } = mixnode;
  const cells: Cell[] = [
    {
      cell: `${stake.amount} ${stake.denom}`,
      id: 'stake-cell',
    },
    {
      cell: `${bond.amount} ${bond.denom}`,
      id: 'bond-cell',
    },
    {
      cell: `${stakeSaturation}%`,
      id: 'stake-saturation-cell',
    },
    {
      cell: `${profitMargin}%`,
      id: 'pm-cell',
    },
    {
      cell: operatorCost ? `${operatorCost} USD` : '-',
      id: 'operator-cost-cell',
    },
    {
      cell: operatorRewards ? `${operatorRewards.amount} ${operatorRewards.denom}` : '-',
      id: 'operator-rewards-cell',
    },
    {
      cell: delegators,
      id: 'delegators-cell',
    },
    {
      cell: (
        <BondedMixnodeActions
          onActionSelect={onActionSelect}
          disabledRedeemAndCompound={(operatorRewards && Number(operatorRewards.amount) === 0) || false}
          disabledBondMore // TODO for now disable bond more feature until backend is ready
        />
      ),
      id: 'actions-cell',
      align: 'right',
    },
  ];

  return (
    <Stack gap={2}>
      <NymCard
        borderless
        title={
          <Stack gap={3}>
            <Box display="flex" alignItems="center" gap={2}>
              <Typography variant="h5" fontWeight={600}>
                Mix node
              </Typography>
              <NodeStatus status={status} />
            </Box>
            {name && (
              <Tooltip title={host} arrow>
                <Typography fontWeight="regular" variant="h6">
                  {name}
                </Typography>
              </Tooltip>
            )}
            <IdentityKey identityKey={identityKey} />
          </Stack>
        }
        Action={
          <Button
            variant="text"
            color="secondary"
            onClick={() => onActionSelect('nodeSettings')}
            startIcon={<NodeIcon />}
          >
            Node Settings
          </Button>
        }
      >
        <NodeTable headers={headers} cells={cells} />
        {network && (
          <Typography sx={{ mt: 2, fontSize: 'small' }}>
            Check more stats of your node on the{' '}
            <Link href={`${urls(network).networkExplorer}/network-components/mixnodes`} target="_blank">
              explorer
            </Link>
          </Typography>
        )}
      </NymCard>
      <NodeStats mixnode={mixnode} />
    </Stack>
  );
};
