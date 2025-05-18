<template>
  <!-- hamburger button -->
  <button
    class="hamburger"
    aria-controls="main-menu"
    :aria-expanded="isOpen"
    @click="toggle"
  >
    <span class="sr-only"></span>
    <svg
      width="24"
      height="24"
      stroke="currentColor"
    >
      <!-- prettier-ignore -->
      <line x1="3" y1="7" x2="21" y2="7" />
      <!-- prettier-ignore -->
      <line x1="3" y1="14" x2="21" y2="14" />
      <!-- prettier-ignore -->
      <line x1="3" y1="21" x2="21" y2="21" />
    </svg>
  </button>

  <!-- off-canvas menu -->
  <Teleport to="body">
    <transition name="slide">
      <nav
        v-if="isOpen"
        id="main-menu"
        ref="menu"
        class="menu"
        @keydown.esc="toggle"
      >
        <a
          v-for="link in menuLinks"
          :key="link.path"
          class="nav-link"
          :href="link.path"
          @click="toggle"
        >
          {{ link.label }}
        </a>
      </nav>
    </transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from "vue";
import { useFocusTrap } from "@vueuse/integrations/useFocusTrap";
import { menuLinks } from "@/constants/navLinks";

const isOpen = ref(false);
function toggle() {
  isOpen.value = !isOpen.value;
}

const menu = ref<HTMLElement | null>(null);
const { activate, deactivate } = useFocusTrap(menu);

onMounted(activate);
onBeforeUnmount(deactivate);
</script>

<style scoped>
@import "@/style.css";

.hamburger {
  position: fixed;
  top: 1rem;
  left: 1rem;
  z-index: 60;
  background: none;
  border: 1px solid rgba(255, 255, 255, 0.1);
  padding: 0.5rem;
  cursor: pointer;
  color: inherit;
}

.menu {
  position: fixed;
  top: 0;
  left: 0;
  height: 100vh;
  width: 16rem;
  padding: 4.5rem 1.5rem 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
  background: #2e2e2e;
  color: inherit;
  box-shadow: 4px 0 8px rgba(0, 0, 0, 0.4);
  border-right: 1px solid rgba(255, 255, 255, 0.1);
}
.nav-link {
  font-size: 1.125rem;
  text-decoration: none;
  color: inherit;
  transition: opacity 0.15s;
}
.nav-link:hover {
  opacity: 0.8;
}

/* slide animation */
.slide-enter-active,
.slide-leave-active {
  transition: transform 0.3s ease;
}
.slide-enter-from,
.slide-leave-to {
  transform: translateX(-100%);
}
.slide-enter-to,
.slide-leave-from {
  transform: translateX(0);
}

@media (prefers-color-scheme: light) {
  .drawer {
    background: #f4f4f4;
    border-right: 1px solid rgba(0, 0, 0, 0.1);
    box-shadow: 4px 0 8px rgba(0, 0, 0, 0.15);
  }
}
</style>
