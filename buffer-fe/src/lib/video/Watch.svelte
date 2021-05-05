<script lang="ts">
  import type { AxiosResponse } from 'axios';
  // noinspection TypeScriptCheckImport
  import { Icon } from 'svelma';
  import { onMount } from 'svelte';
  import { Link } from 'svelte-routing';

  import CommentSection from './CommentSection.svelte';
  import { getVideoDetail, getVideoRating, hasRated, rateVideo } from '../../api/videoApi';
  import { userState } from '../../store/authStore';
  import type { HasRatedDTO, VideoDetailDTO, VideoRatingDTO } from '../../types/dto';

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
  let userHasRated = {
    hasRated: false,
    isDislike: false,
  };
  const ratingHandler = (dislike: boolean) => {
    if (videoId && $userState.signedIn) {
      rateVideo(videoId, dislike)
        .then(() => {
          if (!userHasRated.hasRated) {
            if (!dislike) {
              rating.like++;
            } else {
              rating.dislike++;
            }
            userHasRated.hasRated = true;
          } else {
            if (!dislike) {
              if (!userHasRated.isDislike) {
                rating.like--;
                userHasRated.hasRated = false;
              } else {
                rating.like++;
                rating.dislike--;
              }
            } else {
              if (!userHasRated.isDislike) {
                rating.like--;
                rating.dislike++;
              } else {
                rating.dislike--;
                userHasRated.hasRated = false;
              }
            }
          }
          userHasRated.isDislike = dislike;
        })
        .catch((err) => console.error(err));
    }
  };

  $: if (video.createdAt !== '') {
    let dateObj = new Date(video.createdAt);
    date = dateObj.toDateString();
  }
  $: if ($userState.signedIn && videoId) {
    hasRated(videoId)
      .then((value: AxiosResponse<HasRatedDTO>) => (userHasRated = value.data))
      .catch((err) => console.error(err));
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
      <div class="rating-button" class:has-text-primary={userHasRated.hasRated && !userHasRated.isDislike}>
        <Icon pack="fa" size="is-medium" icon="thumbs-up" on:click={() => ratingHandler(false)} />
      </div>
      {rating.like}/{rating.dislike}
      <div class="rating-button" class:has-text-primary={userHasRated.hasRated && userHasRated.isDislike}>
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
