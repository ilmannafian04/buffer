<script lang="ts">
  import { AxiosResponse } from 'axios';
  // noinspection TypeScriptCheckImport
  import { Button, Field, Icon, Input } from 'svelma';
  import { onMount } from 'svelte';
  import { Link, navigate } from 'svelte-routing';

  import { getCommentsInVideo, newComment } from '../../api/videoApi';
  import { userState } from '../../store/authStore';
  import type { CommentDTO, NewCommentDTO, NewCommentData } from '../../types';
  import { parseDate } from '../../util/stringUtil';

  export let videoId: string;
  let isSubmitting = false;
  let comments: CommentDTO[] = [];
  let commentData: NewCommentData = {
    content: '',
    videoId: '',
  };
  const submitHandler = () => {
    if ($userState.signedIn) {
      newComment(commentData)
        .then((value: AxiosResponse<NewCommentDTO>) => {
          comments = [
            {
              content: value.data.content,
              createdAt: value.data.createdAt,
              id: value.data.id,
              userDisplayName: $userState.user.displayName,
              username: $userState.user.username,
              userId: $userState.user.id,
            },
            ...comments,
          ];
        })
        .catch((err) => console.error(err));
    } else {
      navigate('/signin');
    }
  };
  $: commentData = { ...commentData, videoId: videoId ? videoId : '' };
  const loadMoreComments = (skip: number) => {
    getCommentsInVideo(videoId, skip)
      .then((value: AxiosResponse<CommentDTO[]>) => (comments = [...comments, ...value.data]))
      .catch((err) => console.error(err));
  };
  onMount(() => {
    loadMoreComments(0);
  });
</script>

<form on:submit|preventDefault={submitHandler}>
  <Field label="Create new comment">
    <Input type="textarea" bind:value={commentData.content} />
  </Field>
  <Field>
    {#if isSubmitting}
      <div class="control">
        <Button type="is-primary" loading>{''}</Button>
      </div>
    {:else}
      <div class="control">
        <button class="button is-primary" type="submit">Submit</button>
      </div>
    {/if}
  </Field>
</form>
<ul>
  {#each comments as comment (comment.id)}
    <li>
      <div class="comment py-2">
        <Icon pack="fas" icon="user-circle" size="is-medium" />
        <div>
          <div>
            <Link to="/c/{comment.username}">{comment.userDisplayName}</Link>
            on {parseDate(comment.createdAt)}
          </div>
          <span>{comment.content}</span>
        </div>
      </div>
    </li>
  {/each}
</ul>
<button class="button is-primary" on:click={() => loadMoreComments(comments.length)}>Load more</button>

<style lang="postcss">
  .comment {
    display: flex;
    flex-direction: row;
  }
</style>
