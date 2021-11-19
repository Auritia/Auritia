<template>
  <footer :class="isActive && 'active'" class="flex items-center text-xs w-full px-2 h-6 bg-theme-200">
    <h1 class="flex items-center gap-1" v-if="state.reactive.hint"><i-fluency-bulb />{{ state.reactive.hint }}</h1>
  </footer>
</template>

<script setup lang="ts">
import { useState } from "~/state";

const state = useState();

defineProps<{
  isActive?: boolean;
}>();
</script>

<style scoped lang="postcss">
footer.active {
  @apply bg-accent font-bold text-black;
}
</style>
