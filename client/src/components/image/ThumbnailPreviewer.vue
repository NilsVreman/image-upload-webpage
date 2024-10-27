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
import { computed, onMounted, defineProps } from "vue";
import { useImageStore } from "@/stores/imageStore";
import { storeToRefs } from "pinia";

const props = defineProps<{
  maxThumbnails: {
    type: number;
    default: 100;
  };
}>();

const imageStore = useImageStore();
const { images } = storeToRefs(imageStore);

const displayedImages = computed(() =>
  images.value.slice(0, props.maxThumbnails),
);

onMounted(async () => await imageStore.updateImageMetaData());
</script>

<style scoped>
.thumbnails {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
  gap: 10px;
  margin-top: 10px;
}

.thumbnail {
  position: relative;
  width: 100%;
  padding-top: 100%;
  overflow: hidden;
  border: 1px solid #ccc;
  box-sizing: border-box;
}

.thumbnail img {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  object-fit: cover;
}
</style>
