<script lang="ts">
  // noinspection TypeScriptCheckImport
  import { Field, Input, Button } from 'svelma';

  import { userState } from '../../store/authStore';
  import type { UpdateProfileData } from '../../types/form';
  import { onMount } from 'svelte';
  import { updateProfile } from '../../api/userApi';

  let form;
  let isSubmitting = false;
  let data: UpdateProfileData = {
    displayName: '',
    email: '',
  };
  const submitHandler = () => {
    if (form.reportValidity()) {
      updateProfile(data)
        .then(() => {
          userState.update((prev) => {
            prev.user.displayName = data.displayName;
            prev.user.email = data.email;
            return prev;
          });
        })
        .catch((err) => console.error(err));
    }
  };

  onMount(() => {
    data = {
      displayName: $userState.user.displayName,
      email: $userState.user.email,
    };
  });
</script>

<form bind:this={form}>
  <Field label="Display name">
    <Input bind:value={data.displayName} />
  </Field>
  <Field label="Email">
    <Input type="email" bind:value={data.email} />
  </Field>
  <Field>
    <Button type="is-primary" loading={isSubmitting} on:click={submitHandler}>Submit</Button>
  </Field>
</form>
