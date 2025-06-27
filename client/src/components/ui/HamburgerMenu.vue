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
      fill="none"
      stroke-linecap="round"
      stroke-width="3"
    >
      <!-- prettier-ignore -->
      <line x1="3" y1="5" x2="21" y2="5" />
      <!-- prettier-ignore -->
      <line x1="3" y1="12" x2="21" y2="12" />
      <!-- prettier-ignore -->
      <line x1="3" y1="19" x2="21" y2="19" />
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
import { onClickOutside } from "@vueuse/core";
import { useFocusTrap } from "@vueuse/integrations/useFocusTrap";
import { menuLinks } from "@/constants/navLinks";

const isOpen = ref(false);
function toggle() {
  isOpen.value = !isOpen.value;
}

const menu = ref<HTMLElement | null>(null);
const { activate, deactivate } = useFocusTrap(menu);

onClickOutside(
  menu,
  () => {
    if (isOpen.value) toggle();
  },
  { capture: false },
);

onMounted(activate);
onBeforeUnmount(deactivate);
</script>

<style scoped>
.hamburger {
  position: fixed;
  z-index: 60;
  display: grid;
  place-content: center;
  color: var(--button-color);
  background: transparent;
  border: 2px solid var(--button-color);
  padding: 0.5rem;
  cursor: pointer;
  transition:
    color 0.15s linear,
    border-color 0.15s linear,
    outline-color 0.15s linear,
    box-shadow 0.15s linear;
}

.hamburger:hover {
  color: var(--button-hover-color);
  border-color: var(--button-hover-color);
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
  background: var(--menu-bg);
  color: var(--menu-fg);
  box-shadow: 4px 0 8px rgba(0, 0, 0, 0.4);
  border-right: 1px solid var(--menu-border);
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
.nav-link:first-child {
  padding-top: 1em;
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
</style>
