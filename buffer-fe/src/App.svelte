<script lang="ts">
  import { onMount } from 'svelte';
  import { Router, Route } from 'svelte-routing';
  import '@fortawesome/fontawesome-free/css/all.css';
  import 'bulma/css/bulma.css';

  import { getAccount } from './api/userApi';
  import Signup from './lib/auth/Signup.svelte';
  import Signin from './lib/auth/Signin.svelte';
  import Home from './lib/Home.svelte';
  import { userState } from './store/authStore';
  import Header from './lib/components/Header.svelte';
  import Profile from './lib/creator/Profile.svelte';
  import Watch from './lib/video/Watch.svelte';
  import Upload from './lib/video/Upload.svelte';
  import Account from './lib/creator/Account.svelte';
  import Search from './lib/Search.svelte';
  import Collection from './lib/collection/Collection.svelte';
  import Liked from './lib/video/Liked.svelte';

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
<div class="container my-2">
  <Router url={''}>
    <Route path="/signup">
      <Signup />
    </Route>
    <Route path="/signin">
      <Signin />
    </Route>
    <Route path="/upload">
      <Upload />
    </Route>
    <Route path="/account">
      <Account />
    </Route>
    <Route path="/collection/*">
      <Collection />
    </Route>
    <Route path="/liked">
      <Liked />
    </Route>
    <Route path="/c/:userId" let:params>
      <Profile displayName={params.userId} />
    </Route>
    <Route path="/w/:videoId" let:params>
      <Watch videoId={params.videoId} />
    </Route>
    <Route path="/search" component={Search} />
    <Route path="/">
      <Home />
    </Route>
  </Router>
</div>
