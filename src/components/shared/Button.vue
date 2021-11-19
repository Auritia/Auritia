<template>
  <button
    @mouseenter="hint && state.setHint(hint)"
    @mouseleave="hint && state.setHint('')"
    class="button"
    :class="isActive && 'active'"
  >
    <slot />
  </button>
</template>

<script lang="ts" setup>
import { useState } from "~/state";
const state = useState();
defineProps<{
  hint?: string;
  isActive?: boolean;
}>();
</script>

<style scoped lang="postcss">
.button {
  @apply flex cursor-default text-xs py-0.5 px-2 bg-theme-300 rounded-none;

  &:not(.active) {
    @apply hover:bg-theme-500;
  }

  &.active {
    @apply bg-accent text-black;
  }
}
</style>
