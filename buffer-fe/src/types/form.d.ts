export type SignUpFormData = {
  username: string;
  password: string;
  email: string;
  displayName: string;
};

export type SignInFormData = {
  username: string;
  password: string;
};

export type UploadFormData = {
  title: string;
  description: string;
};

export type NewCommentData = {
  videoId: string;
  content: string;
  isAnonymous: boolean;
};

export type UpdateProfileData = {
  email: string;
  displayName: string;
};

export type NewCollectionData = {
  name: string;
  description: string;
};
