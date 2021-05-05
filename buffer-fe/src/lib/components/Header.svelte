<script lang="ts">
  // noinspection TypeScriptCheckImport
  import { Icon } from 'svelma';
  import { link, navigate } from 'svelte-routing';
  import { userState } from '../../store/authStore';
  import { logout } from '../../util/authUtil';

  let menuIsOpen = false;
  let accountIsOpen = false;
  const toggleMenu = () => (menuIsOpen = !menuIsOpen);
  const closeMenu = () => (menuIsOpen = false);
  const toggleAccount = () => (accountIsOpen = !accountIsOpen);
  const logoutHandler = () => {
    logout();
    closeMenu();
    accountIsOpen = false;
  };
  let navs;
  const pubNavs = [
    {
      name: 'Home',
      path: '/',
    },
  ];
  const authedNavs = [
    {
      name: 'Upload',
      path: '/upload',
    },
  ];
  $: if ($userState.signedIn) {
    navs = [...pubNavs, ...authedNavs];
  } else {
    navs = pubNavs;
  }
</script>

<nav class="navbar" role="navigation">
  <div class="navbar-brand">
    <a class="navbar-item" href="/" use:link>🐺</a>
    <a
      role="button"
      class={menuIsOpen ? 'navbar-burger is-active' : 'navbar-burger'}
      aria-label="menu"
      aria-expanded="false"
      on:click={toggleMenu}
    >
      {#each { length: 3 } as _, i (i)}
        <!-- prettier-ignore -->
        <span aria-hidden='true'></span>
      {/each}
    </a>
  </div>
  <div class={menuIsOpen ? 'navbar-menu is-active' : 'navbar-menu'}>
    <div class="navbar-start">
      {#each navs as navLink (navLink.path)}
        <a class="navbar-item" on:click={closeMenu} use:link href={navLink.path}>{navLink.name}</a>
      {/each}
    </div>

    <div class="navbar-end">
      {#if $userState.signedIn}
        <div class={`navbar-item has-dropdown ${accountIsOpen ? 'is-active' : ''}`} on:click={toggleAccount}>
          <a class="navbar-link">
            <Icon pack="fas" icon="user-circle" size="is-medium" />
          </a>
          <div class="navbar-dropdown is-right">
            <a class="navbar-item" on:click|preventDefault={() => navigate('/account')}>
              {$userState.user.displayName}
            </a>
            <hr class="navbar-divider" />
            <a class="navbar-item" on:click|preventDefault={logoutHandler}>Logout</a>
          </div>
        </div>
      {:else}
        <div class="navbar-item">
          <div class="buttons">
            <a class="button is-primary" href="/signup" on:click={closeMenu} use:link>
              <strong>Sign up</strong>
            </a>
            <a class="button is-light" href="/signin" on:click={closeMenu} use:link>Sign in</a>
          </div>
        </div>
      {/if}
    </div>
  </div>
</nav>
