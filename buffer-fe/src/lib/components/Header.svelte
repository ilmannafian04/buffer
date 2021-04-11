<script lang="ts">
  import { link } from 'svelte-routing';

  let menuIsOpen = false;
  const toggleMenu = () => (menuIsOpen = !menuIsOpen);
  const closeMenu = () => (menuIsOpen = false);
  const navs: { name: string; path: string }[] = [
    {
      name: 'Home',
      path: '/',
    },
  ];
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
        <span aria-hidden="true"></span>
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
      <div class="navbar-item">
        <div class="buttons">
          <a class="button is-primary" href="/signup" on:click={closeMenu} use:link>
            <strong>Sign up</strong>
          </a>
          <a class="button is-light" href="/signin" on:click={closeMenu} use:link>Sign in</a>
        </div>
      </div>
    </div>
  </div>
</nav>
