<template>
  <div
    ref="host"
    class="scrollbar"
    :style="{
      '--active-scale': props.activeScale,
      '--max-height': `${(props.activeScale + 0.25) * props.elementHeight}px`,
    }"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
import { onMounted, onBeforeUnmount, ref } from "vue";

const props = defineProps({
  // height of an element in the scrollbar
  elementHeight: { type: Number, required: true },
  // scale of "active" item (passed into the div)
  activeScale: { type: Number, default: 1.0 },
});

const host = ref<HTMLElement>();
let ticking = false;

// find the child closest to the midpoint
const checkActive = () => {
  if (!host.value) return;
  const box = host.value.getBoundingClientRect();
  const mid = box.left + box.width / 2;
  const children = Array.from(host.value.children) as HTMLElement[];

  const { ele: active } = children.reduce(
    (best, ele) => {
      const rect = ele.getBoundingClientRect();
      const childMid = rect.left + rect.width / 2;
      const dist = Math.abs(mid - childMid);
      return dist < best.dist ? { ele, dist } : best;
    },
    { ele: null as HTMLElement | null, dist: Infinity },
  );

  // with force condition -> removes all .is-active flags except for on el === active
  children.forEach(el => el.classList.toggle("is-active", el === active));
};

// only request rerendering while we're not already rendering
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
  max-width: 100%;
  contain: layout;
  display: flex;
  flex-direction: row;
  align-items: center;
  height: var(--max-height);

  scrollbar-gutter: stable; /* no layout shift */
  overscroll-behavior: contain; /* stop scroll chaining */
  scroll-behavior: smooth;

  overflow-x: auto;
  overflow-y: hidden;
  scroll-snap-type: x mandatory;

  padding-inline: 50%;
}

.scrollbar ::v-deep(*) {
  scroll-snap-align: center;
}

/* any child with .is‑active gets scaled */
.scrollbar ::v-deep(.is-active) {
  transform: scale(var(--active-scale));
  z-index: 1;
}

/* Smooth size transition */
.scrollbar ::v-deep(*) {
  transition: transform 0.25s;
}
</style>
