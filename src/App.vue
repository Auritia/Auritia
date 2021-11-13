<script setup lang="ts">
import Header from "./components/Header.vue";
</script>

<template>
  <div>
    <Header />
    <div class="flex flex-col items-center justify-center w-full h-full">
      <!-- <img src="./assets/full_logo.svg" class="w-96" alt="" /> -->
    </div>
  </div>
</template>

<style lang="postcss">
#app,
body,
html {
  @apply h-full w-full select-none text-white bg-theme-100;
}

:root {
  --accent-color: #d1b1ff;
  --theme-100: #0c0d0f;
  --theme-200: #111215;
  --theme-300: #17191e;
  --theme-400: #1d2026;
  --theme-500: #2a2d34;
  --theme-600: #3e4147;
  --theme-700: #6d6e72;
  --theme-800: #9e9e9e;
  --theme-900: #b6b6b6;
}

svg {
  fill: currentColor;
}

* {
  scrollbar-width: thin;
}
*::selection {
  background-color: var(--accent-color);
}
*::-webkit-scrollbar {
  width: 6px;
  display: none;
  left: -100px;
}
*:hover::-webkit-scrollbar {
  display: initial;
  overflow-y: overlay;
}
*::-webkit-scrollbar-track,
*::-webkit-scrollbar-corner {
  background: transparent;
}
*::-webkit-scrollbar-thumb {
  @apply bg-theme-400;
  border-radius: 20px;
  border: transparent;
}
input[type="range"]::-webkit-slider-runnable-track {
  @apply bg-theme-300 rounded-full h-2;
}
input[type="range"]::-webkit-slider-thumb {
  @apply -mt-1;
}
*.scrollable {
  overflow-y: overlay;
  overflow-x: initial;
}
*.animated {
  @apply duration-100 transition-all;
}
*.clicky {
  @apply animated transform cursor-pointer hover:scale-105 active:scale-95;
}
*.fullscreen {
  @apply absolute z-30 top-0 left-0 w-full h-full;
}
</style>
