import axios from 'axios';
import { derived, writable } from 'svelte/store';

import type { UserState } from '../types/authentication';

export const jwt = writable<string>('');

export const user = derived<typeof jwt, UserState>(jwt, ($jwt) => {
  if ($jwt.length === 0) {
    return {
      signedIn: false,
      user: null,
    };
  } else {
    return {
      signedIn: false,
      user: null,
    };
  }
});
