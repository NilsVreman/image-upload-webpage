<template>
  <HamburgerDrawer :items="navItems" />

  <main class="wrapper">
    <h1>Share your wedding photos</h1>

    <ImageUploader />

    <ThumbnailPreviewer :max-thumbnails="50" />
  </main>
</template>

<script setup lang="ts">
import ImageUploader from "@/components/image/ImageUploader.vue";
import ThumbnailPreviewer from "@/components/image/ThumbnailPreviewer.vue";

const navItems = [
  { label: "Home", to: "/" },
  { label: "Gallery", to: "/gallery" },
];
</script>

<style scoped>
.wrapper {
  width: min(80%, 80vw); /* never exceeds the viewport */
  margin-inline: auto; /* centres horizontally */
  gap: 2rem; /* gap between elements */

  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;

  min-height: 70vh;
}
</style>
