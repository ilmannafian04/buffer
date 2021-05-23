import type { User } from './authentication';

export type SignInResponse = {
  jwt: string;
};

export type CreatorProfileResponse = {
  creator: User;
  followerCount: number;
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
