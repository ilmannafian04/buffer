import axios from 'axios';
import { authenticatedConfig } from './commonApi';
import type { ListVideoDTO } from '../types/dto';
import type { NewCommentData } from '../types/form';

export const uploadVideo = (data: FormData) => {
  let config = authenticatedConfig();
  config.headers['Content-Type'] = 'multipart/form-data';
  return axios.post('/api/a/video/', data, config);
};

export const listVideos = () => {
  return axios.get<ListVideoDTO[]>('/api/video');
};

export const getVideoDetail = (id: string) => {
  return axios.get('/api/video/detail', { params: { id } });
};

export const getCommentsInVideo = (id: string) => {
  return axios.get('/api/video/comments', { params: { id } });
};

export const newComment = (data: NewCommentData) => {
  let config = authenticatedConfig();
  config.headers['Content-Type'] = 'application/json';
  return axios.post('/api/a/video/comment', data, config);
};

export const rateVideo = (id: string, isDislike = false) => {
  return axios.post('/api/a/video/rate', { id, isDislike }, authenticatedConfig());
};

export const getVideoRating = (id: string) => {
  return axios.get('/api/video/rating', { params: { id } });
};

export const hasRated = (id: string) => {
  const config = authenticatedConfig();
  config.params = { id };
  return axios.get('/api/a/video/hasrated', config);
};

export const searchVideo = (term: string) => {
  return axios.get('/api/video/search', { params: { term } });
};
