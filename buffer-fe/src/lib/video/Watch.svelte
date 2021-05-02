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

<vm-player controls>
  <vm-video>
    <source data-src={video.path} type="video/mp4" />
  </vm-video>
</vm-player>
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
