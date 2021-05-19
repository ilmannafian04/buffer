<script lang="ts">
  import { getCollectionDetail } from '../../api/collectionApi';
  import type { CollectionDetailDTO } from '../../types';
  import ListVideo from '../components/ListVideo.svelte';

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
<div>
  {#each collectionDetail.videos as video (video.id)}
    <ListVideo {video} />
  {/each}
</div>
