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
  createdAt: string;
  uploader: string;
  uploaderId: string;
};
