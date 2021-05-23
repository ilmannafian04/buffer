<script lang="ts">
  import type { AxiosResponse } from 'axios';
  // noinspection TypeScriptCheckImport
  import InfiniteScroll from 'svelte-infinite-scroll';

  import VideoRow from '../components/VideoRow.svelte';
  import { getCollectionDetail } from '../../api/collectionApi';
  import type { CollectionAndVideoUsers } from '../../types';

  export let collectionId;
  let collectionDetail: CollectionAndVideoUsers;
  let loadIsFinished = false;
  const loadMore = () => {
    if (!loadIsFinished) {
      getCollectionDetail(collectionId, collectionDetail?.videoUsers.length || 0)
        .then((value: AxiosResponse<CollectionAndVideoUsers>) => {
          if (value.data.videoUsers.length < 5) {
            loadIsFinished = true;
          }
          collectionDetail = {
            ...value.data,
            videoUsers: [...(collectionDetail?.videoUsers || []), ...value.data.videoUsers],
          };
        })
        .catch((err) => console.error(err));
    }
  };
  $: if (collectionId) loadMore();
</script>

{#if collectionDetail}
  <div class="is-size-2">{collectionDetail.collection.name}</div>
  <span>{collectionDetail.collection.description}</span>
  <div>
    {#each collectionDetail.videoUsers as videoUser (videoUser.id)}
      <div class="m-2"><VideoRow {videoUser} /></div>
    {/each}
    <InfiniteScroll window={true} hasMore={!loadIsFinished} on:loadMore={() => loadMore()} />
  </div>
{/if}
