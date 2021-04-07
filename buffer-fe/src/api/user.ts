import axios, { AxiosRequestConfig, AxiosResponse } from 'axios';

import type { User } from '../types/authentication';

const authenticatedConfig = (): AxiosRequestConfig => {
  return {
    headers: {
      Authorization: `Bearer ${localStorage.getItem('jwt')}`,
    },
  };
};

export const getAccount = (): Promise<AxiosResponse<User>> => {
  return axios.get<User>('/api/auth/account', authenticatedConfig());
};
