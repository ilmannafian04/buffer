export type User = {
  username: string;
  email: string;
  displayName: string;
};

export type UserState = {
  signedIn: boolean;
  user: User | null;
};
