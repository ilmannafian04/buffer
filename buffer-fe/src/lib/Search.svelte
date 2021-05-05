<script lang="ts">
  import type { AxiosResponse } from 'axios';

  import ListVideo from './components/ListVideo.svelte';
  import { searchVideo } from '../api/videoApi';
  import type { VideoDetailDTO } from '../types/dto';

  export let term;
  let videos = [];
  $: if (term) {
    searchVideo(term)
      .then((value: AxiosResponse<VideoDetailDTO[]>) => (videos = value.data))
      .catch((err) => console.error(err));
  }
</script>

{#each videos as video (video.id)}
  <ListVideo {video} />
{/each}
