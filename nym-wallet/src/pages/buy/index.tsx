import React from 'react';
import { LoadingModal } from 'src/components/Modals/LoadingModal';
import { Box } from '@mui/material';
import { BuyContextProvider, useBuyContext } from 'src/context';

const Buy = () => {
  const { isLoading } = useBuyContext();

  return <Box sx={{ mt: 4 }}>{isLoading && <LoadingModal />}</Box>;
};

export const BuyPage = () => (
  <BuyContextProvider>
    <Buy />
  </BuyContextProvider>
);
