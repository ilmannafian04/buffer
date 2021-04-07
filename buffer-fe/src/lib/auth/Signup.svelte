<script lang="ts">
  import axios from 'axios';
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

<h1>Sign up</h1>
<form on:submit|preventDefault={handleSubmit}>
  <input type="text" bind:value={formData.username} placeholder="username" disabled={isSubmitting} />
  <input type="password" bind:value={formData.password} placeholder="password" disabled={isSubmitting} />
  <input type="email" bind:value={formData.email} placeholder="email" disabled={isSubmitting} />
  <input type="text" bind:value={formData.displayName} placeholder="displayName" disabled={isSubmitting} />
  <button type="submit" disabled={isSubmitting}>Submit</button>
</form>
