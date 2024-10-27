<template>
  <div class="thumbnails">
    <div
      v-for="(image, index) in displayedImages"
      :key="index"
      class="thumbnail"
    >
      <img
        :src="image.thumbnail_url"
        :alt="image.name"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useImageStore } from "@/stores/imageStore";
import { storeToRefs } from "pinia";

const imageStore = useImageStore();
const { images } = storeToRefs(imageStore);

const displayedImages = computed(() => images.value);

onMounted(async () => await imageStore.updateImageMetaData());
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
