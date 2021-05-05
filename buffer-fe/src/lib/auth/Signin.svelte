<script lang="ts">
  import axios, { AxiosResponse } from 'axios';
  // noinspection TypeScriptCheckImport
  import { Field, Input } from 'svelma';

  import { getAccount } from '../../api/userApi';
  import { userState } from '../../store/authStore';
  import type { SignInResponse } from '../../types/dto';
  import type { SignInFormData } from '../../types/form';

  let isSubmitting = false;
  let initialData: SignInFormData = {
    username: '',
    password: '',
  };
  let formData = initialData;

  const submitHandler = () => {
    // noinspection TypeScriptUnresolvedFunction
    axios
      .post('/api/signin', formData)
      .then((value: AxiosResponse<SignInResponse>) => {
        formData = initialData;
        localStorage.setItem('jwt', value.data.jwt);
        return getAccount();
      })
      .then((value) => {
        userState.set({
          jwt: localStorage.getItem('jwt'),
          signedIn: true,
          user: value.data,
        });
      })
      .catch((err) => console.error(err))
      .finally(() => (isSubmitting = false));
  };
</script>

<div class="is-size-1 pb-2">Sign in</div>
<form on:submit|preventDefault={submitHandler}>
  <Field label="Username">
    <Input type="text" bind:value={formData.username} disabled={isSubmitting} />
  </Field>
  <Field label="Password">
    <Input type="password" bind:value={formData.password} disabled={isSubmitting} />
  </Field>
  <Field>
    <button class="button is-primary" type="submit" disabled={isSubmitting}>Submit</button>
  </Field>
</form>
