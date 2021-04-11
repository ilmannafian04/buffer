import { DEFAULT_STATE, userState } from '../store/auth';

export const logout = () => {
  localStorage.removeItem('jwt');
  userState.set(DEFAULT_STATE);
};
