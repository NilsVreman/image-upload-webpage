<template>
  <HorizontalScrollContainer
    :active-scale="2.0"
    :element-height="100"
  >
    <img
      v-for="(img, idx) in thumbs"
      :key="idx"
      class="thumb"
      :src="img.thumbnail_url"
      :alt="img.name"
      loading="lazy"
    />
  </HorizontalScrollContainer>
</template>

<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useImageStore } from "@/stores/imageStore";
import { storeToRefs } from "pinia";
import HorizontalScrollContainer from "@/components/ui/HorizontalScrollContainer.vue";

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
