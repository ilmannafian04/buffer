<script lang="ts">
  // noinspection TypeScriptCheckImport
  import { Icon } from 'svelma';
  export let title = 'this video';

  let tweet = '';
  $: tweet = `Watch ${title} via Buffer \n${window.location}`;
</script>

<a class="link" href="https://twitter.com/intent/tweet?text={encodeURIComponent(tweet)}" target="_blank">
  <Icon pack="fab" size="is-medium" icon="twitter" />
</a>

<style lang="postcss">
  .link {
    color: inherit;
  }
</style>
