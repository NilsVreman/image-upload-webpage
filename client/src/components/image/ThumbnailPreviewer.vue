<template>
  <ScrollBar direction="horizontal">
    <img
      v-for="(img, idx) in thumbs"
      :key="idx"
      class="thumb"
      :src="img.thumbnail_url"
      :alt="img.name"
      loading="lazy"
    />
  </ScrollBar>
</template>

<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useImageStore } from "@/stores/imageStore";
import { storeToRefs } from "pinia";
import ScrollBar from "@/components/ui/ScrollBar.vue";

/* Images */
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
  flex: 0 0 auto;
  width: 100px;
  height: 100px;
  margin-right: 0.75rem;
  object-fit: cover;
  border: solid 1px black;
  border-radius: 0.25rem;
}
</style>
