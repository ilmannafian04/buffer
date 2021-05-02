<script lang="ts">
  // noinspection TypeScriptCheckImport
  import { Field, Input, Button } from 'svelma';
  import { navigate } from 'svelte-routing';

  import type { UploadFormData } from '../../types/form';
  import { uploadVideo } from '../../api/videoApi';

  let isUploading = false;
  let formData: UploadFormData = {
    description: '',
    title: '',
  };
  let video;
  let thumbnail;
  let submitHandler = () => {
    let data = new FormData();
    data.append('metadata', JSON.stringify({ title: formData.title, description: formData.description }));
    data.append('video', video[0]);
    data.append('thumbnail', thumbnail[0]);
    isUploading = true;
    // noinspection TypeScriptUnresolvedFunction
    uploadVideo(data)
      .then((value) => {
        navigate(`/w/${value.data.id}`);
      })
      .catch((err) => console.error(err))
      .finally(() => (isUploading = false));
  };
</script>

<form on:submit|preventDefault={submitHandler}>
  <Field label="Title">
    <Input type="text" bind:value={formData.title} />
  </Field>
  <Field label="Description">
    <Input type="textarea" bind:value={formData.description} />
  </Field>
  <Field>
    <div class="file has-name">
      <label class="file-label">
        <input class="file-input" type="file" bind:files={video} accept="video/*" />
        <span class="file-cta">
          <span class="file-icon">
            <!--suppress CheckEmptyScriptTag -->
            <i class="fas fa-upload" />
          </span>
          <span class="file-label"> Choose a video… </span>
        </span>
        <span class="file-name">
          {video ? video[0].name : 'Video'}
        </span>
      </label>
    </div>
  </Field>
  <Field>
    <div class="file has-name">
      <label class="file-label">
        <input class="file-input" type="file" bind:files={thumbnail} accept="image/*" />
        <span class="file-cta">
          <span class="file-icon">
            <!--suppress CheckEmptyScriptTag -->
            <i class="fas fa-upload" />
          </span>
          <span class="file-label"> Choose a thumbnail… </span>
        </span>
        <span class="file-name">
          {thumbnail ? thumbnail[0].name : 'Thumbnail'}
        </span>
      </label>
    </div>
  </Field>
  <Field>
    {#if isUploading}
      <div class="control">
        <Button loading type="is-primary">Upload</Button>
      </div>
    {:else}
      <div class="control">
        <button type="submit" class="button is-primary">Upload</button>
      </div>
    {/if}
  </Field>
</form>
