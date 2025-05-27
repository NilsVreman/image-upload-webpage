<template>
  <VueEasyLightbox
    v-if="open"
    :visible="open"
    :imgs="images"
    :index="index"
    :move-disabled="true"
    @hide="$emit('update:open', false)"
  />
</template>

<script setup lang="ts">
import { computed } from "vue";
import VueEasyLightbox from "vue-easy-lightbox/external-css";

const props = defineProps<{
  images: string[];
  imageIndex: number;
  open: boolean;
}>();

defineEmits<{
  (event: "update:open", isLightboxOpen: boolean): void;
}>();

/* Keep the current index reactive inside the component so arrow keys work */
const index = computed({
  get: () => props.imageIndex,
  set: () => {}, // VueEasyLightbox manages it internally
});
</script>

<style scoped>
@import "vue-easy-lightbox/external-css/vue-easy-lightbox.css";
</style>
