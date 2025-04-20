import { defineStore } from "pinia";
import { ref } from "vue";
import api from "@/services/api";
import { acceptedMimeTypes, maximumFileSize } from "@/constants/fileConstants";

export interface ImageMetaData {
  name: string;
  uploaded_at: Date;
  image_url: string;
  thumbnail_url: string;
}

type RawImageMetaData = Omit<ImageMetaData, "uploaded_at"> & {
  uploaded_at: string;
};

const mapToImageMetaData = (images: RawImageMetaData[]): ImageMetaData[] =>
  images.map(image => ({
    ...image,
    uploaded_at: new Date(image.uploaded_at),
  }));

export const useImageStore = defineStore("imageStore", () => {
  const images = ref<ImageMetaData[]>([]);

  const updateImageMetaData = async () => {
    const imageData: ImageMetaData[] = await api
      .get("/images/thumbnails")
      .then(response => mapToImageMetaData(response.data.images));

    const existingNames = new Set(images.value.map(image => image.name));
    const newImages = imageData.filter(image => !existingNames.has(image.name));
    images.value.unshift(...newImages);
  };

  const uploadImages = async (files: File[]) => {
    const formData = new FormData();
    files.forEach(file => formData.append("files", file));

    const uploadedData = await api
      .post("/images", formData, {
        headers: {
          "Content-Type": "multipart/form-data",
        },
      })
      .then(response => mapToImageMetaData(response.data.images));

    console.log("Uploaded images:", uploadedData);
    if (uploadedData.length > 0) {
      console.log("Shifting images");
      images.value.unshift(...uploadedData);
    }
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
