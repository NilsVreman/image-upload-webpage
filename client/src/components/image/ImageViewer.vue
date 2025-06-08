<template>
  <vue-easy-lightbox
    :visible="isOpen"
    :imgs="srcList"
    :index="activeIndex"
    :scroll-disabled="false"
    :move-disabled="true"
    loop
    loading="lazy"
    @hide="close"
    @on-index-change="updateIndex"
  />
</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue";
import VueEasyLightbox from "vue-easy-lightbox";
import type { ImageMetaData } from "@/stores/imageStore";

const emit = defineEmits(["update:modelValue"]);
const props = defineProps<{
  images: ImageMetaData[];
  selectedIndex: number;
  modelValue: boolean;
}>();

const isOpen = ref(props.modelValue);
watch(
  () => props.modelValue,
  newVal => (isOpen.value = newVal),
);
const close = () => (isOpen.value = false);

/* keep caller in sync */
watch(isOpen, newVal => emit("update:modelValue", newVal));

/* active slide */
const activeIndex = ref(props.selectedIndex);
watch(
  () => props.selectedIndex,
  i => (activeIndex.value = i),
);
const updateIndex = (_old: number, newIdx: number) =>
  (activeIndex.value = newIdx);

/* transform to simple src list for lightbox */
const srcList = computed(() => props.images.map(i => i.image_url));
</script>
