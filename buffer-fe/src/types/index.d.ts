export * from './dto';
export * from './form';

export type Collection = {
  id: string;
  name: string;
  description: string;
  videos: any[];
};

export type Video = {
  id: string;
  title: string;
  description: string;
  thumbnailPath: string;
  videoPath: string;
  createdAt: string;
};

export type User = {
  id: string;
  email: string;
  username: string;
  displayName: string;
};

export type VideoUser = {
  video: Video;
  user: User;
};
