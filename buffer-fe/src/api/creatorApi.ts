import axios from 'axios';
import { authenticatedConfig } from './commonApi';

export const creatorProfile = (displayName: string) => {
  return axios.get('/api/creator', {
    params: {
      displayName,
    },
  });
};

export const isFollowing = (displayName: string) => {
  let config = authenticatedConfig();
  config.params = {
    displayName,
  };
  return axios.get<{ isFollowing: boolean }>('/api/a/creator/follow', config);
};

export const follow = (displayName: string) => {
  return axios.post('/api/a/creator/follow', { displayName }, authenticatedConfig());
};

export const unfollow = (displayName: string) => {
  return axios.post('/api/a/creator/unfollow', { displayName }, authenticatedConfig());
};
