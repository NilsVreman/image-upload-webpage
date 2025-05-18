<template>
  <div class="uploader-container">
    <input
      id="image-uploader"
      type="file"
      multiple
      hidden
      :accept="acceptedMimeTypes.join(',')"
      @change="handleFileUpload"
    />
    <label for="image-uploader">
      <button>Select Photos</button>
    </label>
  </div>
</template>

<script setup lang="ts">
import { acceptedMimeTypes } from "@/constants/fileConstants";
import { useImageStore } from "@/stores/imageStore";

const imageStore = useImageStore();

// Handle file selection
const handleFileUpload = async (event: Event) => {
  const target = event.target as HTMLInputElement;
  const validImages = imageStore.filterValidImages(target.files ?? undefined);
  await imageStore.uploadImages(validImages);
};
</script>

<style lang="scss" scoped>
.uploader-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: auto;
}
</style>
