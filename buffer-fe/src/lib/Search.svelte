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

<span class="is-size-2">Search result for {query['term']}</span>
<div class="video-result">
  {#each videos as video (video.id)}
    <div class="m-2">
      <ListVideo {video} />
    </div>
  {/each}
</div>

<style>
  .video-result {
    display: flex;
    flex-wrap: wrap;
    flex-direction: row;
  }
</style>
