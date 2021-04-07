import { writable } from 'svelte/store';

import type { UserState } from '../types/authentication';

export const DEFAULT_STATE = {
  user: null,
  signedIn: false,
  jwt: '',
};

export const userState = writable<UserState>(DEFAULT_STATE);
