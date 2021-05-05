<script lang="ts">
  // noinspection TypeScriptCheckImport
  import { Button, Icon } from 'svelma';
  import { onMount } from 'svelte';
  import { navigate } from 'svelte-routing';

  import type { CreatorProfileResponse } from '../../types/dto';
  import { userState } from '../../store/authStore';
  import { creatorProfile, follow, isFollowing, unfollow } from '../../api/creatorApi';

  export let displayName;
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
        return isFollowing(displayName);
      })
      .then((value) => {
        following = value.data.isFollowing;
      })
      .catch((err) => console.error(err));
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

<div class="name-section">
  <div class="is-size-1 pr-5">
    <Icon pack="fas" icon="user-circle" size="is-medium" />
    &nbsp;{profile.creator.displayName}
  </div>
  <span class="pr-2">{profile.followerCount} followers</span>
  <Button on:click={followHandler}>{following ? 'Followed' : 'Follow'}</Button>
</div>

<style lang="postcss">
  .name-section {
    display: flex;
    align-items: center;
  }
</style>
