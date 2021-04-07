<script lang="ts">
  import axios from 'axios';

  import type { SignInFormData } from 'src/types/form';

  let isSubmitting = false;
  let initialData: SignInFormData = {
    username: '',
    password: '',
  };
  let formData = initialData;

  const submitHandler = () => {
    axios
      .post('/api/signin', formData)
      .then((value) => {
        console.log(value.data);
      })
      .catch((err) => console.error(err));
  };

  $: console.log(formData);
</script>

<h1>Signin</h1>
<form on:submit|preventDefault={submitHandler}>
  <input type="text" bind:value={formData.username} disabled={isSubmitting} />
  <input type="password" bind:value={formData.password} disabled={isSubmitting} />
  <button type="submit" disabled={isSubmitting}>Submit</button>
</form>
