<template>
  <div class="uploader-container">
    <label class="custom-file-upload">
      <div>Upload Images</div>
      <input
        type="file"
        multiple
        :accept="acceptedMimeTypes.join(',')"
        @change="handleFileUpload"
      />
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
@use "@/assets/styles/theme.scss" as theme;

.uploader-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: auto;
}

.custom-file-upload {
  display: inline-block;
  padding: 12px 24px;
  cursor: pointer;
  background-color: theme.$color-button-primary; /* Darker green shade */
  color: theme.$color-text-primary;
  border-radius: 4px;
  position: relative;
  font-size: 16px;
}

.custom-file-upload input[type="file"] {
  position: absolute;
  left: 0;
  top: 0;
  opacity: 0;
  cursor: pointer;
  width: 100%;
  height: 100%;
}
</style>
