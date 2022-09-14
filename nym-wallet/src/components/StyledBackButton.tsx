import React from 'react';
import { Button } from '@mui/material';
import ArrowBackIosNewIcon from '@mui/icons-material/ArrowBackIosNew';

export const StyledBackButton = ({
  onBack,
  label,
  fullWidth,
}: {
  onBack: () => void;
  label?: string;
  fullWidth?: boolean;
}) => (
  <Button disableFocusRipple size="large" fullWidth={fullWidth} variant="outlined" onClick={onBack}>
    {label || <ArrowBackIosNewIcon fontSize="small" />}
  </Button>
);
