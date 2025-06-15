import { defineStore } from "pinia";
import { computed, ref } from "vue";
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

const sortbyDateDesc = (arr: ImageMetaData[]) =>
  [...arr].sort((a, b) => b.uploaded_at.getTime() - a.uploaded_at.getTime());

export const useImageStore = defineStore("imageStore", () => {
  const uploadedImages = ref<ImageMetaData[]>([]);
  const allUploadedImages = ref<ImageMetaData[]>([]);
  const isUploading = ref(false);

  /**
   * exported list:
   *   ‑ uploadedImages first (newest → oldest)
   *   ‑ then the rest from the server (newest → oldest)
   */
  const images = computed<ImageMetaData[]>(() => {
    // Uploaded images by current user
    const uploadedSorted = sortbyDateDesc(uploadedImages.value);
    const uploadedSortedNames = new Set(uploadedSorted.map(img => img.name));

    // Uploaded by all users (not including already sorted images)
    const othersUploaded = sortbyDateDesc(
      allUploadedImages.value.filter(img => !uploadedSortedNames.has(img.name)),
    );

    return [...uploadedSorted, ...othersUploaded];
  });

  const updateImageMetaData = async () => {
    const serverImages: ImageMetaData[] = await api
      .get("/images/thumbnails")
      .then(response => mapToImageMetaData(response.data.images))
      .catch(error => {
        console.error("Error fetching images:", error);
        return [];
      });

    allUploadedImages.value = serverImages;
  };

  const uploadImages = async (files: File[]) => {
    const formData = new FormData();
    files.forEach(file => formData.append("files", file));

    isUploading.value = true;

    try {
      console.log("Uploading images:", files);
      const uploadedData = await api
        .post("/images", formData, {
          headers: {
            "Content-Type": "multipart/form-data",
          },
        })
        .then(response => mapToImageMetaData(response.data.images))
        .catch(error => {
          console.error("Error uploading images:", error);
          return [];
        });

      // Overwrite the uploaded images with the new ones
      uploadedImages.value = uploadedData;

      // Update the server images list
      await updateImageMetaData();
    } finally {
      isUploading.value = false;
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
    isUploading,
  };
});
