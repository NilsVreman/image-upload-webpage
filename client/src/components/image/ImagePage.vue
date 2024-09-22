<template>
  <div class="container">
    <ImageUploader @files-selected="onFilesSelected" />
    <ThumbnailPreviewer :images="images" />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import ImageUploader from "./ImageUploader.vue";
import ThumbnailPreviewer from "./ThumbnailPreviewer.vue";

const images = ref<ThumbnailPreviewer.ImagePreview[]>([]);

const onFilesSelected = (files: File[]) => {
  // Revoke existing image URLs
  images.value.forEach((image) => {
    URL.revokeObjectURL(image.url);
  });
  images.value = [];

  // Create image previews
  files.forEach((file) => {
    const url = URL.createObjectURL(file);
    images.value.push({
      url,
      name: file.name,
    });
  });
};
</script>

<style scoped>
.container {
  flex-direction: column;
  align-items: center;
}
</style>
