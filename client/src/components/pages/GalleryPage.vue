<template>
  <HamburgerMenu />

  <div class="gallery-page">
    <h1>Gallery</h1>

    <GalleryGrid
      :images="images"
      @select-image="openViewer"
    />
    <ImageViewer
      v-model="isViewerOpen"
      :images="images"
      :selected-index="selectedIndex"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useImageStore } from "@/stores/imageStore";
import { storeToRefs } from "pinia";

const imageStore = useImageStore();
const { images } = storeToRefs(imageStore);

const selectedIndex = ref(0);
const isViewerOpen = ref(false);

async function openViewer(index: number) {
  selectedIndex.value = index;
  isViewerOpen.value = true;
}

onMounted(async () => {
  // Fetch the latest image metadata from the server
  await imageStore.updateImageMetaData();
});
</script>

<style scoped>
.gallery-page {
  padding: 1rem;
}
</style>
