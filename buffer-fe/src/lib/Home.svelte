<script lang="ts">
  import type { AxiosResponse } from 'axios';
  import { onMount } from 'svelte';
  import { Link } from 'svelte-routing';

  import { listVideos } from '../api/videoApi';
  import type { ListVideoDTO } from '../types/dto';

  let videos = [];

  onMount(() => {
    listVideos()
      .then((value: AxiosResponse<ListVideoDTO[]>) => {
        videos = value.data;
      })
      .catch((err) => console.error(err));
  });
</script>

<ul>
  {#each videos as video (video.id)}
    <li><Link to="/w/{video.id}">{video.title}</Link></li>
  {/each}
</ul>
