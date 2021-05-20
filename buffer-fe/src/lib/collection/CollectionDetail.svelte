<script lang="ts">
  import type { AxiosResponse } from 'axios';
  // noinspection TypeScriptCheckImport
  import InfiniteScroll from 'svelte-infinite-scroll';

  import ListVideo from '../components/ListVideo.svelte';
  import { getCollectionDetail } from '../../api/collectionApi';
  import type { CollectionDetailDTO } from '../../types';

  export let collectionId;
  let collectionDetail: CollectionDetailDTO = { description: '', name: '', videos: [] };
  let loadIsFinished = false;
  const loadMore = () => {
    if (!loadIsFinished) {
      getCollectionDetail(collectionId, collectionDetail.videos.length)
        .then((value: AxiosResponse<CollectionDetailDTO>) => {
          if (value.data.videos.length === 0) {
            loadIsFinished = true;
          } else {
            collectionDetail = {
              ...value.data,
              videos: [...collectionDetail.videos, ...value.data.videos],
            };
          }
        })
        .catch((err) => console.error(err));
    }
  };
  $: if (collectionId) loadMore();
</script>

<div class="is-size-2">{collectionDetail.name}</div>
<span>{collectionDetail.description}</span>
<div>
  {#each collectionDetail.videos as video (video.id)}
    <div class="m-2"><ListVideo {video} /></div>
  {/each}
  <InfiniteScroll window={true} hasMore={!loadIsFinished} on:loadMore={() => loadMore()} />
</div>
