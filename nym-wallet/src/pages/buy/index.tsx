import React from 'react';
import { LoadingModal } from 'src/components/Modals/LoadingModal';
import { BuyContextProvider, useBuyContext } from 'src/context';
import { Tutorial } from 'src/components/Buy/Tutorial';

const Buy = () => {
  const { isLoading } = useBuyContext();
  if (isLoading) {
    return <LoadingModal />;
  }

  return <Tutorial />;
};

export const BuyPage = () => (
  <BuyContextProvider>
    <Buy />
  </BuyContextProvider>
);
