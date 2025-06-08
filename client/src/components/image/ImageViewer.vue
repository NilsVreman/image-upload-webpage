<template>
  <Dialog
    v-model:visible="isOpen"
    modal
    :closable="false"
  >
    <div class="card galleria-wrapper">
      <Galleria
        v-model:active-index="activeIndex"
        :value="images"
        :responsive-options="responsiveOptions"
        :num-visible="7"
        container-style="max-width: 850px"
        :circular="true"
        :show-item-navigators="true"
        :show-thumbnails="false"
      >
        <template #item="{ item }">
          <DeferredContent @load="null">
            <img
              :src="item.image_url"
              :alt="item.name"
              style="width: 100%; display: block"
              loading="lazy"
            />
          </DeferredContent>
        </template>
      </Galleria>
    </div>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import type { ImageMetaData } from "@/stores/imageStore";

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
.galleria-wrapper {
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
