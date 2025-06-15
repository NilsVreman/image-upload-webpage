<template>
  <router-view />
  <LoadingOverlay :is-overlay-active="isUploading" />
</template>

<script setup lang="ts">
import LoadingOverlay from "@/components/ui/LoadingOverlay.vue";
import { storeToRefs } from "pinia";
import { useImageStore } from "@/stores/imageStore";

const { isUploading } = storeToRefs(useImageStore());
</script>
