<template>
  <div class="relative hover:text-accent h-full text-theme-700">
    <button @click.stop="isShowing = !isShowing" class="h-full">{{ title }}</button>
    <div @click="isShowing = false" v-if="isShowing" class="absolute z-20 flex text-theme-700 items-center bg-theme-100 py-2 flex-col w-72">
      <slot />
    </div>
  </div>
  <div :class="isShowing && 'pointer-events-auto'" class="pointer-events-none z-10 animated opacity-50 fullscreen" @click="isShowing = false"></div>
</template>

<script lang="ts" setup>
import { ref } from "vue";
const isShowing = ref(false);

defineProps<{
  title: string;
}>();
</script>
