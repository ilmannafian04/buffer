<script lang="ts">
  import type { AxiosResponse } from 'axios';
  import queryString from 'query-string';

  import Divider from './components/Divider.svelte';
  import VideoRow from './components/VideoRow.svelte';
  import { searchVideo } from '../api/videoApi';
  import type { VideoUser } from '../types';

  export let location;
  let term;
  let videos = [];
  $: term = queryString.parse(location?.search)['term'];
  $: if (term !== undefined) {
    searchVideo(term)
      .then((value: AxiosResponse<VideoUser[]>) => (videos = value.data))
      .catch((err) => console.error(err));
  }
</script>

<span class="is-size-2">Search result for "{term}"</span>
<Divider />
<div class="video-result">
  {#each videos as video (video.video.id)}
    <div class="m-2">
      <VideoRow videoUser={video} />
    </div>
  {/each}
</div>

<style>
  .video-result {
    display: flex;
    flex-wrap: wrap;
    flex-direction: column;
  }
</style>
