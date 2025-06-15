<template>
  <!-- Render in <body> so the backdrop sits above every route -->
  <teleport to="body">
    <transition name="fade">
      <div
        v-if="isOverlayActive"
        class="loading-modal"
      >
        <div class="loading-modal__spinner"></div>
      </div>
    </transition>
  </teleport>
</template>

<script setup lang="ts">
defineProps<{
  isOverlayActive: boolean;
}>();
</script>

<style scoped>
@keyframes spin-ease {
  to {
    transform: rotate(360deg);
  }
}

/* backdrop */
.loading-modal {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.7);
  z-index: 9999;
}

/* white card */
.loading-modal__spinner {
  position: relative;
  width: 120px;
  height: 120px;
  transition: transform 0.3s;
}

/* scale-up effect during enter/leave */
.fade-enter .loading-modal__spinner,
.fade-leave-to .loading-modal__spinner {
  transform: scale(1.1);
}

/* orange rotating arc */
.loading-modal__spinner::after {
  content: "";
  position: absolute;
  top: calc(50% - 30px);
  left: calc(50% - 30px);
  width: 60px;
  height: 60px;
  border: 8px solid var(--button-color);
  border-radius: 50%;
  border-right-color: transparent;
  border-top-color: transparent;
  animation: spin-ease 0.6s cubic-bezier(0.37, 0, 0.63, 1) infinite;
}

/* fade transition */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
