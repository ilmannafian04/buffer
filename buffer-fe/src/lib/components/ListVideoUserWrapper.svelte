<script lang="ts">
  import ListVideo from './ListVideo.svelte';
  import type { VideoUser } from '../../types';

  export let videoUser: VideoUser;
</script>

{#if videoUser}
  <ListVideo
    video={{
      id: videoUser.video.id,
      title: videoUser.video.title,
      thumbnailPath: videoUser.video.thumbnailPath,
      createdAt: videoUser.video.createdAt,
      uploader: videoUser.user.displayName,
      uploaderId: videoUser.user.username,
    }}
  />
{/if}
