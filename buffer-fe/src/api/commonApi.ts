import type { AxiosRequestConfig } from 'axios';

export const authenticatedConfig = (): AxiosRequestConfig => {
  return {
    headers: {
      Authorization: `Bearer ${localStorage.getItem('jwt')}`,
    },
  };
};
