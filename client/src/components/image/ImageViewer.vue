<template>
  <Galleria
    v-model:visible="isOpen"
    v-model:active-index="activeIndex"
    :value="images"
    :responsive-options="responsiveOptions"
    :num-visible="7"
    :circular="true"
    :show-item-navigators="true"
    :show-thumbnails="false"
    :full-screen="true"
    container-style="width: 80vw; height: 80vh;"
  >
    <template #item="{ item }">
      <DeferredContent @load="null">
        <img
          :src="item.image_url"
          :alt="item.name"
          class="galleria-item"
          loading="lazy"
        />
      </DeferredContent>
    </template>
    <template #thumbnail="{ item }">
      <DeferredContent @load="null">
        <img
          :src="item.thumbnail_url"
          :alt="item.name"
          class="galleria-thumb"
          loading="lazy"
        />
      </DeferredContent>
    </template>
  </Galleria>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import type { ImageMetaData } from "@/stores/imageStore";
import Galleria from "primevue/galleria";
import DeferredContent from "primevue/deferredcontent";

const props = defineProps<{
  images: ImageMetaData[];
  selectedIndex: number;
  modelValue: boolean;
}>();

// Control dialog visibility
const isOpen = ref(props.modelValue);
watch(
  () => props.modelValue,
  newVal => (isOpen.value = newVal),
);

// Keep activeIndex in sync with selectedIndex
const activeIndex = ref(props.selectedIndex);
watch(
  () => props.selectedIndex,
  newIdx => (activeIndex.value = newIdx),
);

// When dialog opens, set Galleria to start at selectedIndex
watch(isOpen, visible => {
  if (visible) activeIndex.value = props.selectedIndex;
});

// Responsive settings: under 960px and 640px, still show 1 item
const responsiveOptions = [
  { breakpoint: "1024px", numVisible: 5 },
  { breakpoint: "768px", numVisible: 3 },
  { breakpoint: "560px", numVisible: 1 },
];
</script>

<style scoped>
/* Full-size image: fits within container while preserving aspect ratio */
.galleria-item {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  margin: auto;
  display: block;
}

/* Hidden thumbnails are not rendered (showThumbnails=false) */
/* Internal thumbnail  */
.galleria-thumb {
  width: 60px;
  height: 60px;
  object-fit: cover;
  border-radius: 0.25rem;
}

/* Close button with a white border around it */
.close-btn {
  position: absolute;
  top: 0.5rem;
  right: 0.5rem;
  background: rgba(0, 0, 0, 0.5);
  border: 2px solid white;
  color: white;
  font-size: 2.5rem;
  width: 2rem;
  height: 2rem;
  line-height: 1.5rem;
  text-align: center;
  cursor: pointer;
  z-index: 2;
}

/* Make the galleria a flex-container */
.p-galleria-fullscreen {
  display: flex;
  justify-content: center;
  align-items: center;
}

/* Force Galleria navigators to overlap the image */
.p-galleria-item-nav {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  z-index: 1;
  background-color: rgba(0, 0, 0, 0.3) !important;
}

/* Shift left navigator to the left edge */
.p-galleria-item-prev {
  left: 0.5rem;
}

/* Shift right navigator to the right edge */
.p-galleria-item-next {
  right: 0.5rem;
}

/* Slightly darken the overlay behind the dialog */
.p-galleria-mask {
  background-color: rgba(0, 0, 0, 0.8) !important;
}
</style>
