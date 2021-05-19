<script lang="ts">
  import { navigate, Route, Router } from 'svelte-routing';
  import NewCollection from './NewCollection.svelte';
</script>

<Router>
  <Route path="/new">
    <NewCollection />
  </Route>
  <Route path="">
    <button class="button is-primary" on:click={() => navigate('/collection/new')}>New Collection</button>
  </Route>
</Router>
