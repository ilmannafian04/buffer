<script lang="ts">
  // noinspection TypeScriptCheckImport
  import { Router, Route } from 'svelte-routing';

  import Signup from './lib/auth/Signup.svelte';
  import Signin from './lib/auth/Signin.svelte';
  import Home from './lib/Home.svelte';
</script>

<Router url={''}>
  <Route path="/signup" component={Signup} />
  <Route path="/signin" component={Signin} />
  <Route path="/" component={Home} />
</Router>

<svelte:head>
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.0.0-beta3/dist/css/bootstrap.min.css" />
</svelte:head>
