<script lang="ts">
  import type { VideoUser } from '../../types';

  export let videoUser: VideoUser;
</script>

{#if videoUser}
  <div class="card">
    <div class="card-image">
      <figure class="image is-16by9">
        <img src={videoUser.video.thumbnailPath} alt="thumbnail" />
      </figure>
    </div>
    <div class="card-content">
      <div class="media">
        <div class="media-content">
          <p class="title is-4">{videoUser.user.displayName}</p>
          <p class="subtitle is-6">{videoUser.user.username}</p>
        </div>
      </div>
      <div class="content">{videoUser.video.description}</div>
    </div>
  </div>
{/if}
