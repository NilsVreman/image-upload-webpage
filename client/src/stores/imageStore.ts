import { defineStore } from "pinia";
import { ref } from "vue";
import api from "@/services/api";
import { acceptedMimeTypes, maximumFileSize } from "@/constants/fileConstants";

export interface ImageMetaData {
  name: string;
  image_url: string;
  thumbnail_url: string;
}

export const useImageStore = defineStore("imageStore", () => {
  const images = ref<ImageMetaData[]>([]);

  const updateImageMetaData = async () => {
    const imageData: ImageMetaData[] = await api
      .get("/images/thumbnails")
      .then(response => response.data.images)
      .catch(err => {
        console.error("Error fetching images:", err);
      });

    const existingImageNames = new Set(images.value.map(image => image.name));
    const newImages = imageData.filter(
      image => !existingImageNames.has(image.name),
    );
    images.value.unshift(...newImages);
  };

  const uploadImages = async (files: File[]) => {
    files.forEach(async file => {
      const formData = new FormData();
      formData.append("files", file);

      await api
        .post("/images", formData, {
          headers: {
            "Content-Type": "multipart/form-data",
          },
        })
        .catch(err => {
          console.error("Error fetching images:", err);
        });
    });

    await updateImageMetaData();
  };

  // Function to select valid files based on MIME type and size
  const filterValidImages = (files?: FileList): File[] =>
    files
      ? [...files].filter(
          ({ type, size }) =>
            acceptedMimeTypes.includes(type) && size <= maximumFileSize,
        )
      : [];

  return {
    images,
    updateImageMetaData,
    uploadImages,
    filterValidImages,
  };
});
