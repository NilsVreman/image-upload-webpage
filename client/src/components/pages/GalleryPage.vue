<template>
  <HamburgerDrawer />

  <main class="gallery">
    <h1>Gallery</h1>

    <GalleryGrid
      :images="images"
      @select="openLightbox"
    />

    <ImageLightbox
      v-model:open="lightboxOpen"
      :images="fullSize"
      :index="lightboxIndex"
    />
  </main>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useImageStore } from "@/stores/imageStore";
import { storeToRefs } from "pinia";

import GalleryGrid from "@/components/image/GalleryGrid.vue";
import ImageLightbox from "@/components/image/ImageLightbox.vue";

const store = useImageStore();
const { images } = storeToRefs(store);
onMounted(async () => await store.updateImageMetaData());

/* lightbox state */
const lightboxOpen = ref(false);
const lightboxIndex = ref(0);
const fullSize = computed(() => images.value.map(i => i.image_url));

function openLightbox(i: number) {
  lightboxIndex.value = i;
  lightboxOpen.value = true;
}
</script>

<style scoped>
.gallery {
  width: min(60rem, 80vw);
  margin-inline: auto;
  padding: 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  min-height: 100vh;
}
</style>
