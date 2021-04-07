<script lang="ts">
  import { Link, navigate } from 'svelte-routing';

  const navs: { name: string; path: string }[] = [
    {
      name: 'Home',
      path: '/',
    },
    {
      name: 'Sign In',
      path: '/signin',
    },
    {
      name: 'Sign Up',
      path: '/signup',
    },
  ];

  const changePage = (path: string) => {
    navigate(path);
  };
</script>

<ul>
  {#each navs as nav (nav.path)}
    <li>
      <a on:click|preventDefault={() => changePage(nav.path)} href="/">{nav.name}</a>
    </li>
  {/each}
</ul>
