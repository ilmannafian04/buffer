<script lang="ts">
  import type { AxiosResponse } from 'axios';
  import { onMount } from 'svelte';

  import VideoCard from '../components/VideoCard.svelte';
  import { listTrendingVideos } from '../../api/videoApi';
  import type { VideoUser } from '../../types';

  export let parentId;
  let videoUsers: VideoUser[] = [];
  onMount(() => {
    listTrendingVideos()
      .then((value: AxiosResponse<VideoUser[]>) => (videoUsers = value.data))
      .catch((err) => console.error(err));
  });
</script>

<div class="root-container">
  {#each videoUsers as videoUser (videoUser)}
    {#if parentId && parentId !== videoUser.video.id}
      <div class="my-1">
        <VideoCard {videoUser} />
      </div>
    {/if}
  {/each}
</div>

<style>
  .root-container {
    display: flex;
    flex-direction: column;
  }
</style>
