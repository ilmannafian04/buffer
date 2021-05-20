<script lang="ts">
  import type { ListVideoDTO } from '../../types';
  import { parseDate } from '../../util/stringUtil';
  import { navigate } from 'svelte-routing';

  export let video: ListVideoDTO;
  const watchVideo = () => {
    navigate(`/w/${video.id}`);
  };
</script>

{#if video}
  <div class="video-container watch-video">
    <img src={video.thumbnailPath} alt="video thumbnail" class="thumbnail mr-2" on:click={() => watchVideo()} />
    <div class="video-detail-container " on:click={() => watchVideo()}>
      <div class="is-size-2">{video.title}</div>
      <div class="is-size-5">Uploaded by {video.uploader}</div>
      <div class="is-size-6">
        on
        {parseDate(video.createdAt)}
      </div>
    </div>
  </div>
{/if}

<style>
  .thumbnail {
    width: 480px;
    height: 270px;
    object-fit: cover;
    object-position: 100% 0;
  }

  .video-container {
    display: flex;
  }

  .video-detail-container {
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .watch-video {
    cursor: pointer;
  }
</style>
