<script lang="ts">
  import { navigate } from 'svelte-routing';

  import type { VideoUser } from '../../types';
  import { parseDate } from '../../util/stringUtil';

  const MAX_DESCRIPTION_LENGTH = 150;
  export let videoUser: VideoUser;
  const watch = () => navigate(`/w/${videoUser.video.id}`);
  const viewUser = () => navigate(`/c/${videoUser.user.username}`);
</script>

{#if videoUser}
  <div class="video-container clickable">
    <!--suppress HtmlUnknownTarget -->
    <img src={videoUser.video.thumbnailPath} alt="video thumbnail" class="thumbnail mr-2" on:click={() => watch()} />
    <div class="video-detail-container " on:click={() => watch()}>
      <div class="is-size-5 video-title my-1">{videoUser.video.title}</div>
      <div class="is-size-6 " on:click|stopPropagation={viewUser}>
        Uploaded by {videoUser.user.displayName}
      </div>
      <div class="is-size-6">
        on
        {parseDate(videoUser.video.createdAt)}
      </div>
      <div class="my-1">
        {videoUser.video.description.slice(0, MAX_DESCRIPTION_LENGTH)}{videoUser.video.description.length >
        MAX_DESCRIPTION_LENGTH
          ? '...'
          : ''}
      </div>
    </div>
  </div>
{/if}

<style>
  .thumbnail {
    width: 336px;
    height: 189px;
    object-fit: cover;
    object-position: 100% 0;
  }

  .video-title {
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    width: 100%;
    display: -webkit-box;
    /*noinspection CssUnknownProperty*/
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
  }

  .video-container {
    display: flex;
  }

  .video-detail-container {
    display: flex;
    flex-direction: column;
  }

  .clickable {
    cursor: pointer;
  }
</style>
