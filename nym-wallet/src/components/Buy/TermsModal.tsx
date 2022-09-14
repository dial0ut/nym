import React from 'react';
import { SimpleModal } from 'src/components/Modals/SimpleModal';
import { Box, Checkbox, FormControlLabel, FormGroup, Typography } from '@mui/material';
import { format } from 'date-fns';
import { ModalDivider } from '../Modals/ModalDivider';

export const TermsModal = ({
  lastUpdated,
  termsText,
  onAccept,
  onDecline,
  onClose,
}: {
  lastUpdated: number;
  termsText: string;
  onAccept: () => Promise<void>;
  onDecline: () => void;
  onClose: () => void;
}) => {
  const [checked, setChecked] = React.useState(false);

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setChecked(event.target.checked);
  };
  const handleOnOK = async () => onAccept();

  return (
    <SimpleModal
      open
      header="Buy NYM Terms and conditions"
      subHeader={`Last updated ${format(lastUpdated, 'dd/MM/yyyy')}`}
      okLabel="Accept"
      okDisabled={!checked}
      onOk={handleOnOK}
      onBack={onDecline}
      backLabel="Decline"
      backButtonFullWidth
      onClose={onClose}
    >
      <Box>
        <ModalDivider sx={{ mb: 2, mt: 2 }} />
        <Typography>{termsText}</Typography>
        <FormGroup>
          <FormControlLabel
            control={<Checkbox checked={checked} onChange={handleChange} inputProps={{ 'aria-label': 'controlled' }} />}
            label="Checkbox"
          />
        </FormGroup>
      </Box>
    </SimpleModal>
  );
};
