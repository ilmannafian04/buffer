<script lang="ts">
  import type { AxiosResponse } from 'axios';
  import { onMount } from 'svelte';

  import VideoCard from './components/VideoCard.svelte';
  import { listVideos } from '../api/videoApi';
  import type { VideoUser } from '../types';

  let videos: VideoUser[] = [];

  onMount(() => {
    listVideos()
      .then((value: AxiosResponse<VideoUser[]>) => {
        videos = value.data;
      })
      .catch((err) => console.error(err));
  });
</script>

<div class="is-size-1 pb-1">New Videos</div>
<div class="listing">
  {#each videos as video (video.video.id)}
    <div class="my-3 mx-2">
      <VideoCard videoUser={video} />
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
