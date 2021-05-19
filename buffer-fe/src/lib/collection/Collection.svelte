<script lang="ts">
  import { navigate, Route, Router } from 'svelte-routing';
  import NewCollection from './NewCollection.svelte';
  import CollectionDetail from './CollectionDetail.svelte';
  import { onMount } from 'svelte';
  import { getMyCollections } from '../../api/collectionApi';
  import type { Collection } from '../../types';

  let collections: Collection[] = [];

  onMount(() => {
    getMyCollections()
      .then((value) => (collections = value.data))
      .catch((err) => console.error(err));
  });
</script>

<Router>
  <Route path="/new">
    <NewCollection />
  </Route>
  <Route path="/:collectionId" let:params>
    <CollectionDetail collectionId={params['collectionId']} />
  </Route>
  <Route path="">
    <button class="button is-primary" on:click={() => navigate('/collection/new')}>New Collection</button>
    {#each collections as collection (collection.id)}
      <div class="my-2">
        <div class="is-size-4">{collection.name} - {collection.videos.length} videos</div>
        <div>{collection.description}</div>
        <button class="button is-primary" on:click={() => navigate(`/collection/${collection.id}`)}>View</button>
      </div>
    {/each}
  </Route>
</Router>
