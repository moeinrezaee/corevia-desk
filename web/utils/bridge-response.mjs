export const isBridgeActionSuccessful = (response) => response?.status === 'accepted'
  && response?.data?.executed !== false
