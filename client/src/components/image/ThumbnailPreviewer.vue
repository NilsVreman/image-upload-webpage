<template>
  <div class="thumbnails">
    <div v-for="(file, index) in files" :key="index" class="thumbnail">
      <img :src="file.thumbnail_url" :alt="file.name" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from "vue";
import axios from "axios";

interface ImageMetaData {
  name: string;
  image_url: string;
  thumbnail_url: string;
}

// TODO: Change this into something more appropriate
const props = withDefaults(
  defineProps<{
    newFiles: ImageMetaData[];
    maximumNumberThumbnails?: number;
  }>(),
  {
    maximumNumberThumbnails: 10,
  },
);

const files = ref<ImageMetaData[]>([]);

// TODO: Change this into something more appropriate
watch(
  // Watch for new files
  () => props.newFiles,
  (newFiles) => {
    newFiles.forEach((file) => {
      const url = URL.createObjectURL(file);
      const newImage: ImageMetaData = {
        image_url: url,
        thumbnail_url: url,
        name: file.name,
      };

      files.value.unshift(newImage);
      console.warn("Don't forget to remove duplicates");

      if (files.value.length > props.maximumNumberThumbnails) {
        const removedFile = files.value.pop();
        URL.revokeObjectURL(removedFile.url);
      }
    });
  },
);

onMounted(async () => {
  await axios
    .get("/api/images")
    .then((response) =>
      response.data.images.forEach((image: ImageMetaData) => {
        files.value.push(image);
      }),
    )
    .catch((error) => {
      console.error("Error fetching images onMounted", error);
    });
});
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
  justify-content: center;
  align-items: center;
  box-sizing: border-box; /* Ensure padding and border are included in width and height */
}

.thumbnail img {
  max-width: 100%;
  max-height: 100%;
}
</style>
