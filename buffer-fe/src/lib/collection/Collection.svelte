<script lang="ts">
  import { navigate, Route, Router } from 'svelte-routing';
  import NewCollection from './NewCollection.svelte';
  import CollectionDetail from './CollectionDetail.svelte';
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
  </Route>
</Router>
