<template>
  <button
    @mouseenter="hint && state.setHint(hint)"
    @mouseleave="hint && state.setHint('')"
    class="hover:text-accent hover:bg-theme-200 py-1 px-6 w-full flex justify-between items-center"
  >
    <p>{{ title }}</p>
    <div class="text-10px flex gap-1">
      <kbd v-for="(shortcut, i) of shortcuts">{{ shortcut }}</kbd>
    </div>
  </button>
</template>

<script lang="ts" setup>
import { useState } from "~/state";
const state = useState();
defineProps<{
  title: string;
  hint?: string;
  shortcuts?: string[];
}>();
</script>
