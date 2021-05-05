<script lang="ts">
  import { onMount } from 'svelte';
  import { searchVideo } from '../api/videoApi';
  import type { AxiosResponse } from 'axios';
  import type { VideoDetailDTO } from '../types/dto';
  import ListVideo from './components/ListVideo.svelte';

  export let term;
  let videos = [];

  onMount(() => {
    searchVideo(term)
      .then((value: AxiosResponse<VideoDetailDTO[]>) => (videos = value.data))
      .catch((err) => console.error(err));
  });
  $: console.log(videos);
</script>

{#each videos as video (video.id)}
  <ListVideo {video} />
{/each}
