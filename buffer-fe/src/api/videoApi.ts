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
