<script lang="ts">
  import { onMount } from 'svelte';
  import { navigate, Route, Router } from 'svelte-routing';

  import AddVideoToCollection from './AddVideoToCollection.svelte';
  import CollectionDetail from './CollectionDetail.svelte';
  import NewCollection from './NewCollection.svelte';
  import { deleteCollection, getMyCollections } from '../../api/collectionApi';
  import type { Collection } from '../../types';

  let collections: Collection[] = [];
  const deleteHandler = (id: string) => {
    deleteCollection(id)
      .then(() => (collections = collections.filter((collection) => collection.id !== id)))
      .catch((err) => console.error(err));
  };

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
  <Route path="/add" component={AddVideoToCollection} />
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
        <button class="button is-danger" on:click={() => deleteHandler(collection.id)}>Delete</button>
      </div>
    {/each}
  </Route>
</Router>
