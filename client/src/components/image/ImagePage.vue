<template>
  <div class="container">
    <ImageUploader />
    <ThumbnailPreviewer :max-thumbnails="50" />
  </div>
</template>

<script setup lang="ts">
import ImageUploader from "./ImageUploader.vue";
import ThumbnailPreviewer from "./ThumbnailPreviewer.vue";
</script>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  align-items: stretch;
  width: 80vw;
  max-width: 100%;
  gap: 10px;
  padding: 20px;
}
</style>
