import React, { useCallback, useEffect, useMemo, useState } from 'react';
import { BuyContext } from '../buy';

export const MockBuyContextProvider = ({ children }: { children?: React.ReactNode }): JSX.Element => {
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string>();

  const resetState = () => {
    setError(undefined);
  };

  const refresh = useCallback(async () => {
    setIsLoading(true);
    // TODO logic
    setIsLoading(false);
  }, []);

  useEffect(() => {
    resetState();
    refresh();
  }, [refresh]);

  const memoizedValue = useMemo(
    () => ({
      isLoading,
      error,
      refresh,
    }),
    [isLoading, error],
  );

  return <BuyContext.Provider value={memoizedValue}>{children}</BuyContext.Provider>;
};
