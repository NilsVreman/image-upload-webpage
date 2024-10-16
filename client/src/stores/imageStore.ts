import { defineStore } from "pinia";
import { ref } from "vue";
import axios from "axios";
import { acceptedMimeTypes, maximumFileSize } from "@/constants/fileConstants";

export interface ImageMetaData {
  name: string;
  image_url: string;
  thumbnail_url: string;
}

export const useImageStore = defineStore("imageStore", () => {
  const images = ref<ImageMetaData[]>([]);
  const isLoading = ref(false);
  const error = ref<string | null>(null);

  const getImageMetaData = async () => {
    isLoading.value = true;
    error.value = null;

    await axios
      .get("/api/images")
      .then((response) => {
        images.value = response.data.images;
      })
      .catch((err) => {
        error.value = err instanceof Error ? err.message : "An error occurred";
        console.error("Error fetching images:", err);
      })
      .finally(() => {
        isLoading.value = false;
      });
  };

  const uploadImages = async (files: File[]) => {
    isLoading.value = true;
    error.value = null;

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
          error.value =
            err instanceof Error ? err.message : "An error occurred";
          uploadedSuccessfully = false;
          console.error("Error fetching images:", err);
        });
    });

    await getImageMetaData();
    isLoading.value = false;
  };

  // Function to select valid files based on MIME type and size
  const filterValidImages = (files?: FileList): File[] => {
    return files
      ? [...files].filter(
          ({ type, size }) =>
            acceptedMimeTypes.includes(type) && size <= maximumFileSize,
        )
      : [];
  };

  return {
    images,
    isLoading,
    error,
    getImageMetaData,
    uploadImages,
    filterValidImages,
  };
});
