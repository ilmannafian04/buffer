export type User = {
  id: string;
  username: string;
  email: string;
  displayName: string;
};

export type UserState = {
  jwt: string;
  signedIn: boolean;
  user: User | null;
};
