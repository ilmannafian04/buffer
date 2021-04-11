import { DEFAULT_STATE, userState } from '../store/authStore';

export const logout = () => {
  localStorage.removeItem('jwt');
  userState.set(DEFAULT_STATE);
};
