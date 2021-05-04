<script lang="ts">
  import type { AxiosResponse } from 'axios';
  import { onMount } from 'svelte';

  import { listVideos } from '../api/videoApi';
  import type { ListVideoDTO } from '../types/dto';
  import ListVideo from './components/ListVideo.svelte';

  let videos = [];

  onMount(() => {
    listVideos()
      .then((value: AxiosResponse<ListVideoDTO[]>) => {
        videos = value.data;
      })
      .catch((err) => console.error(err));
  });
</script>

<div class="is-size-1 pb-1">New Videos</div>
<div class="listing">
  {#each videos as video (video.id)}
    <div class="m-2">
      <ListVideo {video} />
    </div>
  {/each}
</div>

<style lang="postcss">
  .listing {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
  }
</style>
