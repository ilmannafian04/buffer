<script lang="ts">
  import { AxiosResponse } from 'axios';
  // noinspection TypeScriptCheckImport
  import { Field, Icon, Input } from 'svelma';
  import { onMount } from 'svelte';
  import { Link, navigate } from 'svelte-routing';

  import { deleteComment, getCommentsInVideo, newComment } from '../../api/videoApi';
  import { userState } from '../../store/authStore';
  import type { CommentDTO, NewCommentData } from '../../types';
  import { parseDate } from '../../util/stringUtil';

  export let videoId: string;
  let isSubmitting = false;
  let comments: CommentDTO[] = [];
  let loadedAllComments = true;
  let commentData: NewCommentData = {
    content: '',
    videoId: '',
    isAnonymous: false,
  };
  const submitHandler = () => {
    if ($userState.signedIn) {
      newComment(commentData)
        .then((value: AxiosResponse<CommentDTO>) => {
          comments = [value.data, ...comments];
        })
        .catch((err) => console.error(err));
    } else {
      navigate('/signin');
    }
  };
  const deleteHandler = (id: string) => {
    deleteComment(id)
      .then(() => (comments = comments.filter((comment) => comment.id !== id)))
      .catch((err) => console.error(err));
  };
  $: commentData = { ...commentData, videoId: videoId ? videoId : '' };
  const loadMoreComments = (skip: number) => {
    getCommentsInVideo(videoId, skip)
      .then((value: AxiosResponse<CommentDTO[]>) => {
        loadedAllComments = value.data.length < 5;
        comments = [...comments, ...value.data];
      })
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
  <div class="field">
    <label class="checkbox">
      <input type="checkbox" bind:checked={commentData.isAnonymous} />
      Post anonymously
    </label>
  </div>
  <Field>
    <div class="control">
      <button class="button is-primary" class:is-loading={isSubmitting}>Submit</button>
    </div>
  </Field>
</form>
<ul class="my-2">
  {#each comments as comment (comment.id)}
    <li>
      <div class="comment py-2">
        <Icon pack="fas" icon="user-circle" size="is-medium" />
        <div>
          <div class="comment">
            <div>
              {#if comment.isAnonymous}
                Anonymous
              {:else}
                <Link to="/c/{comment.username}">{comment.userDisplayName}</Link>
              {/if}
              on {parseDate(comment.createdAt)}
            </div>
            {#if $userState.user?.username === comment.username}
              <span class="icon is-medium icon-button" on:click={() => deleteHandler(comment.id)}>
                <!-- prettier-ignore -->
                <i class="fa fa-trash" aria-hidden="true"></i>
              </span>
            {/if}
          </div>
          <span>{comment.content}</span>
        </div>
      </div>
    </li>
  {/each}
</ul>
{#if !loadedAllComments}
  <button class="button is-primary" on:click={() => loadMoreComments(comments.length)}>Load more</button>
{/if}

<style lang="postcss">
  .comment {
    display: flex;
    flex-direction: row;
  }
  .icon-button {
    cursor: pointer;
  }
</style>
