import React, { createContext, useCallback, useContext, useEffect, useMemo, useState } from 'react';
// import { Console } from 'src/utils/console';
// import { AppContext } from './main';

export type TBuyContext = {
  isLoading: boolean;
  error?: string;
  refresh: () => Promise<void>;
};

export const BuyContext = createContext<TBuyContext>({
  isLoading: true,
  refresh: async () => undefined,
});

export const BuyContextProvider = ({ children }: { children?: React.ReactNode }): JSX.Element => {
  const [isLoading, setIsLoading] = useState(false);
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

export const useBuyContext = () => useContext<TBuyContext>(BuyContext);
