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
import axios from "axios";

const emit = defineEmits<{
  selectedFiles: (files: File[]) => void;
}>();

// Handle file selection
const handleFileUpload = async (event: Event) => {
  const target = event.target as HTMLInputElement;
  const validFiles = selectValidFiles(target.files);
  const success = await uploadFiles(validFiles);
  if (success) {
    emit("selectedFiles", validFiles);
  }
};

const selectValidFiles = (files?: FileList) =>
  files
    ? [...files].filter(
        ({ type, size }) =>
          acceptedMimeTypes.includes(type) && size < maximumFileSize,
      )
    : undefined;

const uploadFiles = async (files: File[]): Promise<boolean> => {
  let uploadedSuccessfully = true;

  // Upload each file as a separate request to avoid multipart form data size limits
  files.forEach(async (file) => {
    const formData = new FormData();
    formData.append("files", file);

    await axios
      .post("/api/images", formData, {
        headers: {
          "Content-Type": "multipart/form-data",
        },
      })
      .catch((error) => {
        console.error("Error uploading file", error);
        uploadedSuccessfully = false;
      });
  });

  return uploadedSuccessfully;
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
