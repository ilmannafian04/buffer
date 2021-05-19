import axios from 'axios';

export const getCollectionDetail = (id: string) => {
  return axios.get('/api/collection/detail', { params: { id } });
};
