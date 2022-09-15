import React from 'react';
import { Card, CardContent, CardHeader, SxProps } from '@mui/material';
import { styled, Theme } from '@mui/material/styles';
import { Title } from './Title';

const CardContentNoPadding = styled(CardContent)(() => ({
  padding: 0,
  '&:last-child': {
    paddingBottom: 0,
  },
}));

export const NymCard: React.FC<{
  title: string | React.ReactElement;
  subheader?: string;
  Action?: React.ReactNode;
  Icon?: React.ReactNode;
  noPadding?: boolean;
  borderless?: boolean;
  dataTestid?: string;
  sx?: SxProps;
  sxTitle?: SxProps;
}> = ({ title, subheader, Action, Icon, noPadding, borderless, children, dataTestid, sx, sxTitle }) => (
  <Card variant="outlined" sx={{ overflow: 'auto', ...(borderless && { border: 'none', dropShadow: 'none' }), ...sx }}>
    <CardHeader
      sx={{ p: 3, color: (theme: Theme) => theme.palette.text.primary }}
      title={<Title title={title} Icon={Icon} sx={sxTitle} />}
      subheader={subheader}
      data-testid={dataTestid || title}
      subheaderTypographyProps={{ variant: 'subtitle1' }}
      action={Action}
    />
    {noPadding ? (
      <CardContentNoPadding>{children}</CardContentNoPadding>
    ) : (
      <CardContent sx={{ p: 3 }}>{children}</CardContent>
    )}
  </Card>
);
