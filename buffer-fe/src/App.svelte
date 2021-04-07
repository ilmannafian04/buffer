<script lang="ts">
  import { onMount } from 'svelte';
  import { Router, Route } from 'svelte-routing';

  import { getAccount } from './api/user';
  import Signup from './lib/auth/Signup.svelte';
  import Signin from './lib/auth/Signin.svelte';
  import Home from './lib/Home.svelte';
  import { DEFAULT_STATE, userState } from './store/auth';
  import Header from './lib/components/Header.svelte';

  const logoutHandler = () => {
    localStorage.clear();
    userState.set(DEFAULT_STATE);
  };

  onMount(() => {
    let jwt = localStorage.getItem('jwt');
    if (jwt) {
      getAccount()
        .then((value) => {
          userState.set({ jwt, user: value.data, signedIn: true });
        })
        .catch(() => {
          localStorage.removeItem('jwt');
        });
    }
  });
</script>

<Header />
{#if $userState.signedIn}
  <button on:click={logoutHandler}>Logout</button>
{/if}
<Router url={''}>
  <Route path="/signup">
    <Signup />
  </Route>
  <Route path="/signin">
    <Signin />
  </Route>
  <Route path="/">
    <Home />
  </Route>
</Router>

<svelte:head>
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.0.0-beta3/dist/css/bootstrap.min.css" />
</svelte:head>
