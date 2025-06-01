<template>
  <div class="gallery-grid">
    <!--eslint-disable-->
    <div
      v-for="(img, idx) in images"
      :key="img.name"
      class="grid-item"
      @click="$emit('select-image', idx)"
    >
      <!--eslint-enable-->
      <DeferredContent @load="null">
        <img
          :src="img.thumbnail_url"
          :alt="img.name"
          class="thumb"
          loading="lazy"
        />
      </DeferredContent>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ImageMetaData } from "@/stores/imageStore";
import DeferredContent from "primevue/deferredcontent";

defineProps<{
  images: ImageMetaData[];
}>();
</script>

<style scoped>
.gallery-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
  gap: 0.75rem;
  padding: 1rem;
}
.grid-item {
  position: relative;
  cursor: pointer;
}
.thumb {
  width: 100%;
  aspect-ratio: 1/1;
  object-fit: cover;
  cursor: pointer;
  border-radius: 0.25rem;
  transition: opacity 0.15s;
}
.thumb:hover {
  opacity: 0.7;
}
</style>
