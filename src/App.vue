<script setup lang="ts">
import Header from "./components/MenuBar.vue";
import Navigation from "./components/Navigation.vue";
</script>

<template>
  <div class="flex flex-col h-full w-full">
    <Header />
    <div class="flex flex-col bg-theme-300 justify-center w-full overflow-scroll h-full">
      <!-- <img src="./assets/full_logo.svg" class="w-96" alt="" /> -->
      <Navigation />
      <div class="w-full"></div>
    </div>
    <div class="flex h-96"></div>
  </div>
</template>

<style lang="postcss">
#app,
body,
html {
  @apply h-full w-full select-none text-white bg-theme-100;
  font-family: "Roboto";
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
  --theme-800: #a0a0a0;
  --theme-900: #b9b9b9;
}

svg {
  fill: currentColor;
}

* {
  @apply !outline-none;
}
*::selection {
  background-color: var(--accent-color);
}
*::-webkit-scrollbar {
  width: 6px;
  display: none;
  left: -100px;
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
.scrollable {
  overflow-y: overlay;
  overflow-x: initial;
}
.animated {
  @apply duration-50 transition-all;
}
.clicky {
  @apply cursor-pointer;
}
.fullscreen {
  @apply absolute top-0 left-0 w-full h-full;
}
</style>
