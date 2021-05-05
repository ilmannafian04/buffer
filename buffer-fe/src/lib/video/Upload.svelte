<script lang="ts">
  // noinspection TypeScriptCheckImport
  import { Field, Input, Button } from 'svelma';
  // noinspection TypeScriptCheckImport
  import Dropzone from 'svelte-file-dropzone';
  import { navigate } from 'svelte-routing';

  import type { UploadFormData } from '../../types/form';
  import { uploadVideo } from '../../api/videoApi';

  let isUploading = false;
  let isValid = false;
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
  const dropHandler = (e) => {
    e.detail.acceptedFiles.forEach((currentFile) => {
      if (currentFile.type === 'video/mp4') {
        video = [currentFile];
      } else {
        thumbnail = [currentFile];
      }
    });
  };
  $: {
    let valid = true;
    if (formData.title.length === 0) valid = false;
    if (video === undefined) valid = false;
    if (thumbnail === undefined) valid = false;
    isValid = valid;
  }
</script>

<form>
  <Field label="Title">
    <Input type="text" bind:value={formData.title} />
  </Field>
  <Field label="Description">
    <Input type="textarea" bind:value={formData.description} />
  </Field>
  <Field>
    <Dropzone accept={['video/mp4', 'image/png']} on:drop={dropHandler}>
      <span>{video ? `Selected video: ${video[0].name}` : 'Select a video'}</span>
      <span>{thumbnail ? `Selected thumbnail: ${thumbnail[0].name}` : 'Select a thumbnail'}</span>
    </Dropzone>
  </Field>
  <Field>
    <div class="control">
      <Button loading={isUploading} type="is-primary" on:click={submitHandler} disabled={!isValid}>Upload</Button>
    </div>
    <!--{#if isUploading}-->
    <!--  <div class="control">-->
    <!--    <Button loading type="is-primary">Upload</Button>-->
    <!--  </div>-->
    <!--{:else}-->
    <!--  <div class="control">-->
    <!--    <button type="submit" class="button is-primary">Upload</button>-->
    <!--  </div>-->
    <!--{/if}-->
  </Field>
</form>
