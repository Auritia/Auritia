<template>
  <button class="button" :class="isActive && 'active'">
    <slot />
  </button>
</template>

<script lang="ts" setup>
defineProps<{
  isActive: boolean;
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
