<script lang="ts">
  import axios from 'axios';
  // noinspection TypeScriptCheckImport
  import { Field, Input } from 'svelma';
  import { navigate } from 'svelte-routing';

  import type { SignUpFormData } from '../../types/form';

  let isSubmitting = false;
  let defaultData: SignUpFormData = {
    username: '',
    password: '',
    email: '',
    displayName: '',
  };
  let formData: SignUpFormData = defaultData;

  const handleSubmit = () => {
    isSubmitting = true;
    // noinspection TypeScriptUnresolvedFunction
    axios
      .post('/api/signup', formData)
      .then(() => {
        navigate('/signin');
        formData = defaultData;
      })
      .catch((err) => console.error(err))
      .finally(() => (isSubmitting = false));
  };
</script>

<div class="is-size-1 pb-2">Sign up</div>
<form on:submit|preventDefault={handleSubmit}>
  <Field label="Username">
    <Input type="text" bind:value={formData.username} disabled={isSubmitting} />
  </Field>
  <Field label="Password">
    <Input type="password" bind:value={formData.password} disabled={isSubmitting} />
  </Field>
  <Field label="Email">
    <Input type="email" bind:value={formData.email} disabled={isSubmitting} />
  </Field>
  <Field label="Display name">
    <Input type="text" bind:value={formData.displayName} disabled={isSubmitting} />
  </Field>
  <Field>
    <div class="control"><button class="button is-primary" type="submit" disabled={isSubmitting}>Submit</button></div>
  </Field>
</form>
