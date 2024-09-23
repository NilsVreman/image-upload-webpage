<template>
  <div class="container">
    <ImageUploader @selected-files="selectedFiles = $event" />
    <ThumbnailPreviewer
      :new-files="selectedFiles"
      :maximum-number-thumbnails="10"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import ImageUploader from "./ImageUploader.vue";
import ThumbnailPreviewer from "./ThumbnailPreviewer.vue";

const selectedFiles = ref<File[]>([]);
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
