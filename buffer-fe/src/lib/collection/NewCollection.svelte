<script lang="ts">
  import type { NewCollectionData } from '../../types/form';
  import { newCollection } from '../../api/videoApi';
  import { navigate } from 'svelte-routing';

  let isSubmitting = false;
  let formData: NewCollectionData = {
    description: '',
    name: '',
  };
  let submitHandler = () => {
    if (!isSubmitting) {
      newCollection(formData)
        .then((value) => navigate(`/collection/${value.data['id']}`))
        .catch((err) => console.error(err));
    }
  };
</script>

<form on:submit|preventDefault={submitHandler}>
  <label class="label">Name</label>
  <div class="field">
    <input class="input" type="text" bind:value={formData.name} />
  </div>
  <label class="label">Description</label>
  <div class="field">
    <!-- prettier-ignore -->
    <textarea class="textarea" bind:value={formData.description}></textarea>
  </div>
  <div class="control">
    <button class="button is-primary" class:is-loading={isSubmitting} type="submit">Submit</button>
  </div>
</form>
