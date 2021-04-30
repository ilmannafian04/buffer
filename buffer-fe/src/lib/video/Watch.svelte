<script lang="ts">
  import type { AxiosResponse } from 'axios';
  import { onMount } from 'svelte';
  import { Link } from 'svelte-routing';

  import { getVideoDetail } from '../../api/videoApi';
  import type { VideoDetailDTO } from '../../types/dto';

  export let videoId;
  let video: VideoDetailDTO = {
    createdAt: '',
    description: '',
    id: '',
    path: '',
    title: '',
    uploader: '',
    uploaderId: '',
  };

  onMount(() => {
    getVideoDetail(videoId)
      .then((value: AxiosResponse<VideoDetailDTO>) => {
        video = value.data;
      })
      .catch((err) => console.error(err));
  });
</script>

<div>video goes here</div>
<div>
  <div class="is-size-4">{video.title}</div>
  <div class="creator-detail">
    <span class="is-size-4">🧑</span>
    <Link to="/c/{video.uploader}">{video.uploader}</Link>
    <span>Uploaded on {video.createdAt}</span>
  </div>
  <div>
    {video.description}
  </div>
</div>

<style lang="postcss">
  .creator-detail {
    display: flex;
    align-items: center;
  }
</style>
