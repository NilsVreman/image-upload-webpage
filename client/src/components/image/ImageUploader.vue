<template>
  <button
    type="button"
    class="upload-btn"
    @click="openFileHandler"
  >
    Select photos
  </button>

  <input
    ref="fileInput"
    type="file"
    multiple
    hidden
    class="sr-only"
    :accept="acceptedMimeTypes.join(',')"
    @change="handleFileUpload"
  />
</template>

<script setup lang="ts">
import { ref } from "vue";
import { acceptedMimeTypes } from "@/constants/fileConstants";
import { useImageStore } from "@/stores/imageStore";

const imageStore = useImageStore();
const fileInput = ref<HTMLInputElement | null>(null);

const openFileHandler = () => fileInput.value?.click();

// Handle file selection
const handleFileUpload = async (event: Event) => {
  const newFiles = (event.target as HTMLInputElement).files ?? undefined;
  const validImages = imageStore.filterValidImages(newFiles);
  await imageStore.uploadImages(validImages);

  (event.target as HTMLInputElement).value = ""; // Reset input value
};
</script>
