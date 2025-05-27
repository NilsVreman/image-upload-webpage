<template>
  <div class="grid">
    <img
      v-for="(img, idx) in images"
      :key="img.name"
      :src="img.thumbnail_url"
      :alt="img.name"
      class="thumb"
      loading="lazy"
      @click="$emit('select', idx)"
    />
  </div>
</template>

<script setup lang="ts">
import type { ImageMetaData } from "@/stores/imageStore";

defineProps<{
  images: ImageMetaData[];
}>();

defineEmits<{
  (event: "select", imageIndex: number): void;
}>();
</script>

<style scoped>
.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
  gap: 0.75rem;
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
