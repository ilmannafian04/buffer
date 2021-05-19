<script lang="ts">
  import ListVideo from '../components/ListVideo.svelte';
  import { getCollectionDetail } from '../../api/collectionApi';
  import type { CollectionDetailDTO } from '../../types';

  export let collectionId;
  let collectionDetail: CollectionDetailDTO = { description: '', name: '', videos: [] };
  $: if (collectionId) {
    getCollectionDetail(collectionId)
      .then((value) => (collectionDetail = value.data))
      .catch((err) => console.error(err));
  }
</script>

<div class="is-size-2">{collectionDetail.name}</div>
<span>{collectionDetail.description}</span>
<div class="video-section">
  {#each collectionDetail.videos as video (video.id)}
    <div class="m-2"><ListVideo {video} /></div>
  {/each}
</div>

<style>
  .video-section {
    display: flex;
    flex-wrap: wrap;
    flex-direction: row;
  }
</style>
