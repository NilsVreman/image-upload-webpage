<template>
  <div class="thumbnails">
    <div v-for="(image, index) in images" :key="index" class="thumbnail">
      <img :src="image.url" :alt="image.name" />
    </div>
  </div>
</template>

<script setup lang="ts">
interface ImagePreview {
  url: string;
  name: string;
}

defineProps<{
  images: ImagePreview[];
}>();
</script>

<style scoped>
.thumbnails {
  display: flex;
  flex-wrap: wrap;
  margin-top: 20px;
  gap: 10px;
}

.thumbnail {
  max-width: 150px;
  max-height: 150px;
  border: 1px solid #ccc;
  padding: 5px;
  display: flex;
  justify-content: center; /* Horizontally center */
  align-items: center; /* Vertically center */
  box-sizing: border-box; /* Ensure padding and border are included in width and height */
}

.thumbnail img {
  max-width: 100%;
  max-height: 100%;
}
</style>
