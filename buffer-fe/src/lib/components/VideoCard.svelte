<script lang="ts">
  import type { VideoUser } from '../../types';
  import { navigate } from 'svelte-routing';
  import { parseDate } from '../../util/stringUtil';

  export let videoUser: VideoUser;
  const watch = () => {
    if (videoUser) navigate(`/w/${videoUser.video.id}`);
  };
</script>

{#if videoUser}
  <div class="clickable" on:click={watch}>
    <!--suppress HtmlUnknownTarget -->
    <img src={videoUser.video.thumbnailPath} alt="thumbnail" class="thumbnail" />
    <div class="video-title">{videoUser.video.title}</div>
    <div class="creator-name">{videoUser.user.displayName}</div>
    <div class="video-detail">
      {parseDate(videoUser.video.createdAt)}
    </div>
  </div>
{/if}

<style>
  .thumbnail {
    width: 272px;
    height: 153px;
  }
  .video-title {
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    width: 272px;
    display: -webkit-box;
    /*noinspection CssUnknownProperty*/
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
  }
  .video-detail {
    font-weight: lighter;
  }
  .clickable {
    cursor: pointer;
  }
  .creator-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
