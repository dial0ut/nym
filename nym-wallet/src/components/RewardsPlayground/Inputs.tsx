import React from 'react';
import { yupResolver } from '@hookform/resolvers/yup';
import { Button, Grid, TextField, Typography } from '@mui/material';
import { useForm } from 'react-hook-form';
import { inputValidationSchema } from './inputsValidationSchema';
import { useBondingContext } from 'src/context';

export type InputFields = {
  label: string;
  name: 'profitMargin' | 'uptime' | 'bond' | 'delegations' | 'operatorCost';
  isPercentage?: boolean;
}[];

const inputFields: InputFields = [
  { label: 'Profit margin', name: 'profitMargin', isPercentage: true },
  { label: 'Operator cost', name: 'operatorCost' },
  { label: 'Bond', name: 'bond' },
  { label: 'Delegations', name: 'delegations' },
  { label: 'Uptime', name: 'uptime', isPercentage: true },
];

export const Inputs = ({ onCalculate }: { onCalculate: () => Promise<void> }) => {
  const { bondedNode } = useBondingContext();

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm({
    resolver: yupResolver(inputValidationSchema),
    defaultValues: {
      profitMargin: bondedNode?.profitMargin || '',
      uptime: '',
      bond: bondedNode?.bond.amount || '',
      delegations: '',
      operatorCost: '',
    },
  });

  return (
    <Grid container spacing={3}>
      {inputFields.map((field) => (
        <Grid item xs={12} lg={2}>
          <TextField
            {...register(field.name)}
            fullWidth
            label={field.label}
            name={field.name}
            error={Boolean(errors[field.name])}
            helperText={errors[field.name]?.message}
            InputProps={{
              endAdornment: <Typography sx={{ color: 'grey.600' }}>{field.isPercentage ? '%' : 'NYM'}</Typography>,
            }}
          />
        </Grid>
      ))}{' '}
      <Grid item xs={12} lg={2}>
        <Button
          variant="contained"
          disableElevation
          onClick={handleSubmit(onCalculate)}
          size="large"
          fullWidth
          disabled={Boolean(Object.keys(errors).length)}
        >
          Calculate
        </Button>
      </Grid>
    </Grid>
  );
};
