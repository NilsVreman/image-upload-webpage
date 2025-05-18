<template>
  <VueEasyLightbox
    v-if="open"
    :visible="open"
    :imgs="images"
    :index="index"
    @hide="$emit('update:open', false)"
  />
</template>

<script setup lang="ts">
import { computed } from "vue";
import VueEasyLightbox from "vue-easy-lightbox";

const props = defineProps<{
  images: string[];
  index: number;
  open: boolean;
}>();

defineEmits<{
  (e: "update:open", value: boolean): void;
}>();

/* Keep the current index reactive inside the component so arrow keys work */
const index = computed({
  get: () => props.index,
  set: () => {}, // VueEasyLightbox manages it internally
});
</script>
