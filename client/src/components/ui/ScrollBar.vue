<template>
  <div
    ref="host"
    class="scrollbar"
    :class="axisClass"
    :style="{ '--active-scale': props.activeScale }"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onBeforeUnmount, ref } from "vue";

const props = defineProps({
  direction: {
    type: String,
    default: "horizontal",
    validator: v => ["horizontal", "vertical"].includes(v),
  },

  // scale of "active" item (passed into the div)
  activeScale: { type: Number, default: 2.0 },
});

const axisClass = computed(() =>
  props.direction === "vertical"
    ? "scrollbar--vertical"
    : "scrollbar--horizontal",
);

const host = ref<HTMLElement>();
let ticking = false;

// When scrolling stops (or throttled), find the child closest to centre
const checkActive = () => {
  if (!host.value) return;
  const box = host.value.getBoundingClientRect();

  const mid =
    props.direction === "vertical"
      ? box.top + box.height / 2
      : box.left + box.width / 2;

  const children = Array.from(host.value.children) as HTMLElement[];

  // Find the closest child to the mid-point
  const { ele: active } = children.reduce(
    (best, ele) => {
      const rect = ele.getBoundingClientRect();
      const childMid =
        props.direction === "vertical"
          ? rect.top + rect.height / 2
          : rect.left + rect.width / 2;
      const dist = Math.abs(mid - childMid);
      return dist < best.dist ? { ele, dist } : best;
    },
    { ele: null as HTMLElement | null, dist: Infinity },
  );

  // With force condition -> removes all .is-active flags except for on el === active
  children.forEach(el => el.classList.toggle("is-active", el === active));
};

// Only request rerendering while we're not rendering
const onScroll = () => {
  if (ticking) return;
  ticking = true;
  requestAnimationFrame(() => {
    checkActive();
    ticking = false;
  });
};

onMounted(() =>
  host.value?.addEventListener("scroll", onScroll, { passive: true }),
);
onBeforeUnmount(() => host.value?.removeEventListener("scroll", onScroll));
</script>

<style scoped>
.scrollbar {
  scrollbar-gutter: stable; /* no layout shift */
  overscroll-behavior: contain; /* stop scroll chaining */
  scroll-behavior: smooth;
}

.scrollbar--horizontal {
  overflow-x: auto;
  overflow-y: hidden;
  white-space: nowrap;
  scroll-snap-type: x mandatory;
}
.scrollbar--vertical {
  overflow-y: auto;
  overflow-x: hidden;
  scroll-snap-type: y mandatory;
}

.scrollbar ::v-deep(*) {
  scroll-snap-align: center;
}

/* Any child with .is‑active gets scaled -- consumers may override */
.scrollbar ::v-deep(.is-active) {
  transform: scale(var(--active-scale));
}
/* Smooth size transition */
.scrollbar ::v-deep(*) {
  transition: transform 0.25s;
}
</style>
