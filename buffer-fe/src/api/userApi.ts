import axios, { AxiosResponse } from 'axios';

import { authenticatedConfig } from './commonApi';
import type { User } from '../types/authentication';

export const getAccount = (): Promise<AxiosResponse<User>> => {
  return axios.get<User>('/api/auth/account', authenticatedConfig());
};
