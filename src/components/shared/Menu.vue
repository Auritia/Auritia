<template>
  <div class="menu relative h-full">
    <button @click.stop="isShowing = !isShowing" class="hover:text-accent h-full">{{ title }}</button>
    <div @click="isShowing = false" v-if="isShowing" class="absolute z-20 flex items-center bg-theme-100 py-2 flex-col w-72">
      <slot />
    </div>
  </div>
  <div
    :class="isShowing && 'pointer-events-auto'"
    class="pointer-events-none z-10 animated opacity-50 fullscreen"
    @click="isShowing = false"
  ></div>
</template>

<script lang="ts" setup>
import { onKeyStroke } from "@vueuse/core";
import { ref } from "vue";
const isShowing = ref(false);
onKeyStroke("Escape", () => (isShowing.value = false));

defineProps<{
  title: string;
}>();
</script>
