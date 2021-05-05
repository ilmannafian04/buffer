<script lang="ts">
  import type { AxiosResponse } from 'axios';
  // noinspection TypeScriptCheckImport
  import { Icon } from 'svelma';
  import { onMount } from 'svelte';
  import { Link } from 'svelte-routing';

  import CommentSection from './CommentSection.svelte';
  import { getVideoDetail, getVideoRating, rateVideo } from '../../api/videoApi';
  import type { VideoDetailDTO, VideoRatingDTO } from '../../types/dto';

  export let videoId;
  let date = '';
  let video: VideoDetailDTO = {
    createdAt: '',
    description: '',
    id: '',
    path: '',
    thumbnailPath: '',
    title: '',
    uploader: '',
    uploaderId: '',
  };
  let rating = {
    like: 0,
    dislike: 0,
  };
  const ratingHandler = (dislike: boolean) => {
    if (videoId) {
      rateVideo(videoId, dislike)
        .then(() => {
          if (dislike) {
            rating.dislike++;
            rating.like--;
          } else {
            rating.like++;
            rating.dislike--;
          }
        })
        .catch((err) => console.error(err));
    }
  };

  $: {
    if (video.createdAt !== '') {
      let dateObj = new Date(video.createdAt);
      date = dateObj.toDateString();
    }
  }

  onMount(() => {
    getVideoDetail(videoId)
      .then((value: AxiosResponse<VideoDetailDTO>) => {
        video = value.data;
        return getVideoRating(videoId);
      })
      .then((value: AxiosResponse<VideoRatingDTO>) => (rating = value.data))
      .catch((err) => console.error(err));
  });
</script>

<vm-player controls>
  <vm-video poster={video.thumbnailPath}>
    <source data-src={video.path} type="video/mp4" />
  </vm-video>
</vm-player>
<div>
  <div class="is-size-4">{video.title}</div>
  <div class="creator-detail">
    <div><Link to="/c/{video.uploader}">{video.uploader}</Link> uploaded on {date}</div>
    <div class="rating">
      <div class="rating-button">
        <Icon pack="fa" size="is-medium" icon="thumbs-up" on:click={() => ratingHandler(false)} />
      </div>
      {rating.like}/{rating.dislike}
      <div class="rating-button">
        <Icon pack="fa" size="is-medium" icon="thumbs-down" on:click={() => ratingHandler(true)} />
      </div>
    </div>
  </div>
  <div>
    {video.description}
  </div>
</div>
<hr class="divider" />
<CommentSection videoId={video.id} />

<style lang="postcss">
  .creator-detail {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .divider {
    border-top: 1px solid #bbb;
  }
  .rating {
    display: flex;
    align-items: center;
  }
  .rating-button {
    cursor: pointer;
  }
</style>
