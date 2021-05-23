import axios from 'axios';

import { authenticatedConfig, jsonAuthedConfig } from './commonApi';
import type { NewCollectionData, NewCommentData } from '../types';

export const uploadVideo = (data: FormData) => {
  let config = authenticatedConfig();
  config.headers['Content-Type'] = 'multipart/form-data';
  return axios.post('/api/a/video/', data, config);
};

export const listVideos = () => {
  return axios.get('/api/video');
};

export const getVideoDetail = (id: string) => {
  return axios.get('/api/video/detail', { params: { id } });
};

export const getCommentsInVideo = (id: string, skip = 0) => {
  return axios.get('/api/video/comments', { params: { id, skip } });
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

export const getVideoByCreator = (username: string) => {
  return axios.get('/api/creator/videos', { params: { username } });
};

export const newCollection = (data: NewCollectionData) => {
  return axios.post('/api/a/collection/new', data, jsonAuthedConfig());
};

export const getLikedVideos = () => {
  return axios.get('/api/a/video/liked', authenticatedConfig());
};

export const deleteVideo = (id: string) => {
  return axios.post('/api/a/video/delete', { id }, jsonAuthedConfig());
};

export const deleteComment = (id: string) => {
  return axios.post('/api/a/video/comment/delete', { id }, jsonAuthedConfig());
};
