import axios from 'axios';
import { authenticatedConfig } from './commonApi';

export const uploadVideo = (data: FormData) => {
  let config = authenticatedConfig();
  config.headers['Content-Type'] = 'multipart/form-data';
  return axios.post('/api/a/video/', data, config);
};
