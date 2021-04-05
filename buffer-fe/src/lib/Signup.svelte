<script lang="ts">
  import axios from 'axios';

  import type { SignUpFormData } from '../types/SignUp';

  let formData: SignUpFormData = {
    username: '',
    password: '',
    email: '',
    displayName: '',
  };

  const handleSubmit = () => {
    axios
      .post('/api/signup', formData)
      .then((value) => {
        console.log(value);
      })
      .catch((err) => console.error(err));
  };

  $: console.log(formData);
</script>

<h1>Sign up</h1>
<form on:submit|preventDefault={handleSubmit}>
  <input type="text" bind:value={formData.username} placeholder="username" />
  <input type="password" bind:value={formData.password} placeholder="password" />
  <input type="email" bind:value={formData.email} placeholder="email" />
  <input type="text" bind:value={formData.displayName} placeholder="displayName" />
  <button type="submit">Submit</button>
</form>
