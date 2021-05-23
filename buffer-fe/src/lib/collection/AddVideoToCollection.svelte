<script lang="ts">
  import queryString from 'query-string';

  import { addVideoToCollection, getMyCollections } from '../../api/collectionApi';
  import type { Collection } from '../../types';
  import { navigate } from 'svelte-routing';

  export let location;
  let param;
  let isSubmitting = false;
  let selected;
  let collections: Collection[] = [];
  $: param = queryString.parse(location?.search);
  $: if (param && param['id']) {
    getMyCollections()
      .then((value) => (collections = value.data))
      .catch((err) => console.error(err));
  }
  const submitHandler = () => {
    isSubmitting = true;
    if (param && param['id'] && selected) {
      // noinspection TypeScriptUnresolvedFunction
      addVideoToCollection(selected, param['id'])
        .then(() => navigate(`/collection/${selected}`))
        .catch((err) => console.error(err))
        .finally(() => (isSubmitting = false));
    }
  };
</script>

<form on:submit|preventDefault={submitHandler}>
  <div class="field">
    <label class="label">Select collection</label>
    <div class="control">
      <div class="select">
        <select bind:value={selected}>
          <option value={undefined}>Select collection</option>
          {#each collections as collection (collection.id)}
            <option value={collection.id}>{collection.name}</option>
          {/each}
        </select>
      </div>
    </div>
  </div>
  <div class="control">
    <button class="button is-primary" class:is-loading={isSubmitting} type="submit">Add</button>
  </div>
</form>
