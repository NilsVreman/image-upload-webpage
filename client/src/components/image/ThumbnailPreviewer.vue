<template>
  <div class="thumbnails">
    <div
      v-for="(file, index) in files"
      :key="index"
      class="thumbnail"
    >
      <img
        :src="file.thumbnail_url"
        :alt="file.name"
      />
    </div>
  </div>
</template>
<!--- <template>
  <div class="thumbnails">
    <div v-if="isLoading">Loading...</div>
    <div v-else>
      <div
        v-for="(file, index) in files"
        :key="index"
        class="thumbnail"
      >
        <img
          :src="file.thumbnail_url"
          :alt="file.name"
        />
      </div>
    </div>
    <div
      v-if="error"
      class="error"
    >
      {{ error }}
    </div>
  </div>
</template> ---->

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
  grid-template: auto / repeat(5, minmax(100px, 1fr));
  gap: 10px;
  margin-top: 10px;
}

.thumbnail {
  max-width: 100px;
  max-height: 100px;
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
