<template>
  <HorizontalScrollContainer
    :active-scale="2.0"
    :element-height="100"
  >
    <!--
      Browsers can chose to ignore `loading="lazy"` since the <img> tag has dimensions 0x0 at startup (since they can all fit inside the viewport).
      That's why we set the width and height attributes to 100x100: https://web.dev/articles/browser-level-image-lazy-loading#dimension-attributes
    -->
    <img
      v-for="(img, idx) in thumbs"
      :key="idx"
      class="thumb"
      :src="img.thumbnail_url"
      :alt="img.name"
      width="100"
      height="100"
      loading="lazy"
    />
  </HorizontalScrollContainer>
</template>

<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useImageStore } from "@/stores/imageStore";
import { storeToRefs } from "pinia";

const props = defineProps({
  maxThumbnails: {
    type: Number,
    default: 100,
  },
});

const imageStore = useImageStore();
const { images } = storeToRefs(imageStore);

const thumbs = computed(() => images.value.slice(0, props.maxThumbnails));

onMounted(async () => await imageStore.updateImageMetaData());
</script>

<style scoped>
.thumb {
  width: 100px;
  height: 100px;
  margin: 0.5rem;
  object-fit: cover;
  border: solid 1px black;
  border-radius: 0.25rem;
}
</style>
