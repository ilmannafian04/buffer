import { writable } from 'svelte/store';

import type { UserState } from '../types/authentication';

export const userState = writable<UserState>({
  user: null,
  signedIn: false,
  jwt: '',
});
