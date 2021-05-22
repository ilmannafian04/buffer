import type { User } from './authentication';

export type SignInResponse = {
  jwt: string;
};

export type CreatorProfileResponse = {
  creator: User;
  followerCount: number;
};

export type ListVideoDTO = {
  id: string;
  title: string;
  thumbnailPath: string;
  createdAt: string;
  uploader: string;
  uploaderId: string;
};

export type VideoDetailDTO = {
  id: string;
  title: string;
  description: string;
  path: string;
  thumbnailPath: string;
  createdAt: string;
  uploader: string;
  uploaderUsername: string;
  uploaderId: string;
};

export type CommentDTO = {
  id: string;
  content: string;
  createdAt: string;
  userId: string;
  userDisplayName: string;
  username: string;
  isAnonymous: boolean;
};

export type VideoRatingDTO = {
  like: number;
  dislike: number;
};

export type HasRatedDTO = {
  hasRated: boolean;
  isDislike: boolean;
};

export type CollectionDetailDTO = {
  name: string;
  description: string;
  videos: VideoDetailDTO[];
};
