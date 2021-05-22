<script lang="ts">
  import { navigate } from 'svelte-routing';

  import type { ListVideoDTO } from '../../types';

  export let video: ListVideoDTO;
  let date = '';
  $: {
    if (video) {
      let dateObj = new Date(video.createdAt);
      date = dateObj.toDateString();
    }
  }
  const clickHandler = () => {
    navigate(`/w/${video.id}`);
  };
</script>

<div class="root" on:click={clickHandler}>
  <!--suppress HtmlUnknownTarget -->
  <img src={video.thumbnailPath} alt="Thumbnail" class="thumbnail pb-1" />
  <div class="pb-1">{video.title}</div>
  <div class="extras">
    <div class="pr-1">{video.uploader}</div>
    <div>{date}</div>
  </div>
</div>

<style lang="postcss">
  .root {
    display: flex;
    flex-direction: column;
    clear: both;
    cursor: pointer;
  }

  .thumbnail {
    width: 320px;
    height: 180px;
    object-fit: cover;
    object-position: 100% 0;
  }

  .extras {
    display: flex;
    flex-direction: row;
  }
</style>
