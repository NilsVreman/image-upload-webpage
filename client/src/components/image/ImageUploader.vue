<template>
  <div class="uploader-container">
    <label class="custom-file-upload">
      Upload Images
      <input
        type="file"
        multiple
        :accept="acceptedMimeTypes"
        @change="handleFileUpload"
      />
    </label>
  </div>
</template>

<script setup lang="ts">
import { acceptedMimeTypes, maximumFileSize } from "@/constants/fileConstants";

const emit = defineEmits<{
  filesSelected: (files: File[]) => void;
}>();

// Handle file selection
const handleFileUpload = (event: Event) => {
  const target = event.target as HTMLInputElement;
  if (target.files) {
    const validFiles = Array.from(target.files).filter((file) => {
      return fileTypeFilter(file) && fileSizeFilter(file);
    });

    emit("filesSelected", validFiles);
  }
};

const fileTypeFilter = (file: File) => {
  return acceptedMimeTypes.includes(file.type);
};
const fileSizeFilter = (file: File) => {
  return file.size < maximumFileSize;
};
</script>

<style lang="scss" scoped>
@import "@/assets/styles/theme";

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
  background-color: $color-button-primary; /* Darker green shade */
  color: $color-text-primary;
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
