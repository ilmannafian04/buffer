import axios from 'axios';
import { authenticatedConfig } from './commonApi';

export const creatorProfile = (username: string) => {
  return axios.get('/api/creator', {
    params: {
      username,
    },
  });
};

export const isFollowing = (username: string) => {
  let config = authenticatedConfig();
  config.params = {
    username,
  };
  return axios.get<{ isFollowing: boolean }>('/api/a/creator/follow', config);
};

export const follow = (username: string) => {
  return axios.post('/api/a/creator/follow', { username }, authenticatedConfig());
};

export const unfollow = (username: string) => {
  return axios.post('/api/a/creator/unfollow', { username }, authenticatedConfig());
};
