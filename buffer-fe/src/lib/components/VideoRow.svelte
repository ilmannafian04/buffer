<script lang="ts">
  import { navigate } from 'svelte-routing';

  import type { VideoUser } from '../../types';
  import { parseDate } from '../../util/stringUtil';

  export let videoUser: VideoUser;
  const watchVideo = () => {
    navigate(`/w/${videoUser.video.id}`);
  };
</script>

{#if videoUser}
  <div class="video-container watch-video">
    <!--suppress HtmlUnknownTarget -->
    <img
      src={videoUser.video.thumbnailPath}
      alt="video thumbnail"
      class="thumbnail mr-2"
      on:click={() => watchVideo()}
    />
    <div class="video-detail-container " on:click={() => watchVideo()}>
      <div class="is-size-2">{videoUser.video.title}</div>
      <div class="is-size-5">Uploaded by {videoUser.user.displayName}</div>
      <div class="is-size-6">
        on
        {parseDate(videoUser.video.createdAt)}
      </div>
      <div>{videoUser.video.description}</div>
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
