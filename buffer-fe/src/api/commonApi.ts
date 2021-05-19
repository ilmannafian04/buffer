import type { AxiosRequestConfig } from 'axios';

export const authenticatedConfig = (): AxiosRequestConfig => {
  return {
    headers: {
      Authorization: `Bearer ${localStorage.getItem('jwt')}`,
    },
  };
};

export const jsonAuthedConfig = (): AxiosRequestConfig => {
  const config = authenticatedConfig();
  config.headers['Content-Type'] = 'application/json';
  return config;
};
