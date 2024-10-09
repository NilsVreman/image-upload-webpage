<template>
  <div class="container">
    <ImageUploader />
    <ThumbnailPreviewer />
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
  align-items: center;
  gap: 10px;
  padding: 20px;
}
</style>
