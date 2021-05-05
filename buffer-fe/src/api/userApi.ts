import axios, { AxiosResponse } from 'axios';

import { authenticatedConfig } from './commonApi';
import type { User } from '../types/authentication';
import type { UpdateProfileData } from '../types/form';

export const getAccount = (): Promise<AxiosResponse<User>> => {
  return axios.get<User>('/api/auth/account', authenticatedConfig());
};

export const updateProfile = (data: UpdateProfileData) => {
  return axios.post('/api/a/creator/profile', data, authenticatedConfig());
};
