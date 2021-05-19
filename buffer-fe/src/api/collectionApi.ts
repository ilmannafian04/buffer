import axios from 'axios';
import { authenticatedConfig } from './commonApi';

export const getCollectionDetail = (id: string) => {
  return axios.get('/api/collection/detail', { params: { id } });
};

export const getMyCollections = () => {
  return axios.get('/api/a/collection/byme', authenticatedConfig());
};
