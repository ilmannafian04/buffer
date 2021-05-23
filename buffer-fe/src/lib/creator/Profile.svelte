<script lang="ts">
  import type { AxiosResponse } from 'axios';
  // noinspection TypeScriptCheckImport
  import { Button, Icon } from 'svelma';
  import { onMount } from 'svelte';
  import { navigate } from 'svelte-routing';

  import { creatorProfile, follow, isFollowing, unfollow } from '../../api/creatorApi';
  import { getVideoByCreator } from '../../api/videoApi';
  import { userState } from '../../store/authStore';
  import type { CreatorProfileResponse, VideoUser } from '../../types';
  import VideoCard from '../components/VideoCard.svelte';

  export let displayName;
  let videos: VideoUser[] = [];
  let profile: CreatorProfileResponse = {
    creator: {
      id: '',
      displayName: '',
      email: '',
      username: '',
    },
    followerCount: 0,
  };
  let following = false;

  onMount(() => {
    creatorProfile(displayName)
      .then((value) => {
        profile = value.data;
        return getVideoByCreator(displayName);
      })
      .then((value: AxiosResponse<VideoUser[]>) => (videos = value.data))
      .catch((err) => console.error(err));
    if ($userState.signedIn) {
      isFollowing(displayName)
        .then((value: AxiosResponse<{ isFollowing: boolean }>) => (following = value.data.isFollowing))
        .catch((err) => console.error(err));
    }
  });

  let followHandler = () => {
    if ($userState.signedIn) {
      if (following) {
        unfollow(displayName)
          .then(() => {
            profile.followerCount -= 1;
            following = false;
          })
          .catch((err) => console.error(err));
      } else {
        follow(displayName)
          .then(() => {
            profile.followerCount += 1;
            following = true;
          })
          .catch((err) => console.error(err));
      }
    } else {
      navigate('/signin');
    }
  };
</script>

<div class="name-section pb-2">
  <div class="is-size-1 pr-5">
    <Icon pack="fas" icon="user-circle" size="is-medium" />
    &nbsp;{profile.creator.displayName}
  </div>
  <span class="pr-2">{profile.followerCount} followers</span>
  <Button on:click={followHandler}>{following ? 'Followed' : 'Follow'}</Button>
</div>
<div class="video-section">
  {#each videos as video (video.video.id)}
    <div class="pr-4 pb-2"><VideoCard videoUser={video} /></div>
  {/each}
</div>

<style lang="postcss">
  .name-section {
    display: flex;
    align-items: center;
  }

  .video-section {
    display: flex;
    flex-wrap: wrap;
  }
</style>
