<template>
  <div class="thumbnails">
    <div v-for="(file, index) in files" :key="index" class="thumbnail">
      <img :src="file.url" :alt="file.name" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";

interface ImagePreview {
  url: string;
  name: string;
}

const props = withDefaults(
  defineProps<{
    newFiles: ImagePreview[];
    maximumNumberThumbnails?: number;
  }>(),
  {
    maximumNumberThumbnails: 10,
  },
);

const files = ref<ImagePreview[]>([]);

watch(
  () => props.newFiles,
  (newFiles) => {
    newFiles.forEach((file) => {
      const url = URL.createObjectURL(file);
      const newImage: ImagePreview = { url, name: file.name };

      files.value.unshift(newImage);
      console.warn("Don't forget to remove duplicates");

      if (files.value.length > props.maximumNumberThumbnails) {
        const removedFile = files.value.pop();
        URL.revokeObjectURL(removedFile.url);
      }
    });
  },
);
</script>

<style scoped>
.thumbnails {
  display: grid;
  gap: 10px;
  margin-top: 10px;
}

@media (min-width: 1200px) {
  .thumbnails {
    grid-template-columns: repeat(5, 1fr);
    grid-template-rows: repeat(2, 1fr);
  }
}

@media (min-width: 800px) and (max-width: 1199px) {
  .thumbnails {
    grid-template-columns: repeat(4, 1fr);
    grid-template-rows: repeat(3, 1fr);
  }
}

@media (min-width: 600px) and (max-width: 799px) {
  .thumbnails {
    grid-template-columns: repeat(3, 1fr);
    grid-template-rows: repeat(4, 1fr);
  }
}

@media (max-width: 599px) {
  .thumbnails {
    grid-template-columns: repeat(2, 1fr);
    grid-template-rows: repeat(5, 1fr);
  }
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
