<script lang="ts">
  import type { AxiosResponse } from 'axios';
  // noinspection TypeScriptCheckImport
  import { Icon } from 'svelma';
  import { onMount } from 'svelte';
  import { Link, navigate } from 'svelte-routing';

  import CommentSection from './CommentSection.svelte';
  import VideoSuggestion from './VideoSuggestion.svelte';
  import ShareToTwitter from '../components/ShareToTwitter.svelte';
  import { deleteVideo, getVideoDetail, getVideoRating, hasRated, rateVideo } from '../../api/videoApi';
  import { userState } from '../../store/authStore';
  import type { HasRatedDTO, VideoRatingDTO, VideoUser } from '../../types';
  import { parseDate } from '../../util/stringUtil';

  export let videoId;
  let videoUser: VideoUser;
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
  const handleDelete = () => {
    deleteVideo(videoId)
      .then(() => navigate('/'))
      .catch((err) => console.error(err));
  };

  $: if ($userState.signedIn && videoId) {
    hasRated(videoId)
      .then((value: AxiosResponse<HasRatedDTO>) => (userHasRated = value.data))
      .catch((err) => console.error(err));
  }
  $: if (videoId) {
    getVideoDetail(videoId)
      .then((value: AxiosResponse<VideoUser>) => {
        videoUser = value.data;
        return getVideoRating(videoId);
      })
      .then((value: AxiosResponse<VideoRatingDTO>) => (rating = value.data))
      .catch((err) => console.error(err));
  }
</script>

{#if videoUser !== undefined}
  <div class="under-video">
    <div class="comment-section mr-3">
      <vm-player controls>
        <vm-video poster={videoUser.video.thumbnailPath}>
          <source data-src={videoUser.video.videoPath} type="video/mp4" />
        </vm-video>
      </vm-player>
      <div>
        <div class="is-size-4">{videoUser.video.title}</div>
        <div class="creator-detail">
          <div>
            <Link to="/c/{videoUser.user.username}">{videoUser.user.displayName}</Link>
            uploaded on {parseDate(videoUser.video.createdAt)} - {videoUser.video.viewCount} views
          </div>
          <div class="info-right">
            {#if $userState.user?.id === videoUser.user.id}
              <span class="icon is-medium icon-button" on:click={handleDelete}>
                <!-- prettier-ignore -->
                <i class="fa fa-trash" aria-hidden="true"></i>
              </span>
            {/if}
            {#if $userState.signedIn}
              <span class="icon is-medium icon-button" on:click={() => navigate(`/collection/add?id=${videoId}`)}>
                <!-- prettier-ignore -->
                <i class="fa fa-plus" aria-hidden="true"></i>
              </span>
            {/if}
            <ShareToTwitter title={videoUser.video.title} />
            <div class="icon-button" class:has-text-primary={userHasRated.hasRated && !userHasRated.isDislike}>
              <Icon pack="fa" size="is-medium" icon="thumbs-up" on:click={() => ratingHandler(false)} />
            </div>
            {rating.like}/{rating.dislike}
            <div class="icon-button" class:has-text-primary={userHasRated.hasRated && userHasRated.isDislike}>
              <Icon pack="fa" size="is-medium" icon="thumbs-down" on:click={() => ratingHandler(true)} />
            </div>
          </div>
        </div>
        <div>
          {videoUser.video.description}
        </div>
      </div>
      <hr class="divider" />
      <CommentSection {videoId} />
    </div>
    <VideoSuggestion parentId={videoUser.video.id} />
  </div>
{/if}

<style lang="postcss">
  .comment-section {
    flex-grow: 1;
  }
  .under-video {
    display: flex;
    flex-flow: row nowrap;
  }
  .creator-detail {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .divider {
    border-top: 1px solid #bbb;
  }

  .info-right {
    display: flex;
    align-items: center;
  }

  .icon-button {
    cursor: pointer;
  }
</style>
