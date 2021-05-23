<script lang="ts">
  import type { AxiosResponse } from 'axios';
  import { onMount } from 'svelte';
  import { navigate } from 'svelte-routing';

  import VideoCard from '../components/VideoCard.svelte';
  import { getLikedVideos } from '../../api/videoApi';
  import { userState } from '../../store/authStore';
  import type { VideoUser } from '../../types';

  let videos: VideoUser[] = [];

  onMount(() => {
    if (!$userState.signedIn) {
      navigate('/signin');
    } else {
      getLikedVideos()
        .then((value: AxiosResponse<VideoUser[]>) => (videos = value.data))
        .catch((err) => console.error(err));
    }
  });
</script>

<div class="is-size-2">Liked by me</div>
<div class="list-container">
  {#each videos as video (video.video.id)}
    <div class="m-2"><VideoCard videoUser={video} /></div>
  {/each}
</div>

<style>
  .list-container {
    display: flex;
  }
</style>
