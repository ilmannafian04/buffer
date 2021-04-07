<script lang="ts">
  import axios, { AxiosResponse } from 'axios';

  import { userState } from '../../store/auth';
  import type { User } from '../../types/authentication';
  import type { SignInResponse } from '../../types/dto';
  import type { SignInFormData } from '../../types/form';

  let isSubmitting = false;
  let initialData: SignInFormData = {
    username: '',
    password: '',
  };
  let formData = initialData;

  const submitHandler = () => {
    axios
      .post('/api/signin', formData)
      .then((value: AxiosResponse<SignInResponse>) => {
        formData = initialData;
        userState.update((prev) => {
          return {
            ...prev,
            jwt: value.data.jwt,
          };
        });
        return axios.get<User>('/api/auth/account', {
          headers: {
            Authorization: `Bearer ${value.data.jwt}`,
          },
        });
      })
      .then((value) => {
        userState.update((prev) => {
          return {
            ...prev,
            signedIn: true,
            user: value.data,
          };
        });
      })
      .catch((err) => console.error(err))
      .finally(() => (isSubmitting = false));
  };
</script>

<h1>Signin</h1>
<form on:submit|preventDefault={submitHandler}>
  <input type="text" bind:value={formData.username} disabled={isSubmitting} />
  <input type="password" bind:value={formData.password} disabled={isSubmitting} />
  <button type="submit" disabled={isSubmitting}>Submit</button>
</form>
