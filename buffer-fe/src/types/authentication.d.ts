export type User = {
  username: string;
  email: string;
  displayName: string;
};

export type UserState = {
  jwt: string;
  signedIn: boolean;
  user: User | null;
};
