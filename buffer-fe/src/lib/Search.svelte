<script lang="ts">
  import type { AxiosResponse } from 'axios';
  import queryString from 'query-string';

  import ListVideo from './components/ListVideo.svelte';
  import { searchVideo } from '../api/videoApi';
  import type { VideoDetailDTO } from '../types/dto';

  export let location;
  let query;
  let videos = [];
  $: query = queryString.parse(location?.search);
  $: if (query && query['term']) {
    searchVideo(query['term'])
      .then((value: AxiosResponse<VideoDetailDTO[]>) => (videos = value.data))
      .catch((err) => console.error(err));
  }
</script>

{#each videos as video (video.id)}
  <ListVideo {video} />
{/each}
