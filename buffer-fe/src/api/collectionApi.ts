import axios from 'axios';
import { authenticatedConfig, jsonAuthedConfig } from './commonApi';

export const getCollectionDetail = (id: string) => {
  return axios.get('/api/collection/detail', { params: { id } });
};

export const getMyCollections = () => {
  return axios.get('/api/a/collection/byme', authenticatedConfig());
};

export const addVideoToCollection = (collectionId: string, videoId: string) => {
  return axios.post('/api/a/collection/addvideo', { collectionId, videoId }, jsonAuthedConfig());
};
