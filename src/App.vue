<script setup lang="ts">
import MenuBar from "./components/MenuBar.vue";
import Navigation from "./components/Navigation.vue";
import Header from "./components/Header.vue";
import Footer from "./components/Footer.vue";
import Timeline from "./components/Timeline.vue";
import { useState } from "./state/index";
const state = useState();
</script>

<template>
  <div class="flex flex-col h-full w-full">
    <MenuBar />
    <Header />
    <div class="flex bg-theme-300 justify-center w-full overflow-scroll h-full">
      <Navigation />
      <Timeline />
    </div>
    <div class="flex">
      <Footer :text="state.reactive.hint"> <i-fluency-bulb /> </Footer>
      <Footer
        @click="state.reactive.errorText = undefined"
        class="flex-1 bg-red-600 whitespace-nowrap text-white"
        :text="state.reactive.errorText"
      >
        <i-fluency-file />
      </Footer>
    </div>
  </div>
</template>

<style lang="postcss">
#app,
body,
html {
  @apply text-theme-700 h-full w-full select-none bg-theme-100;
  font-family: "Roboto";
}

kbd {
  margin: 0px 0.1em;
  padding: 0.1em 0.6em;
  border-radius: 2px;
  border: 1px solid var(--theme-200);
  color: var(--theme-700);
  line-height: 1.4;
  font-family: Arial, Helvetica, sans-serif;
  font-size: 10px;
  display: inline-block;
  box-shadow: 0px 1px 0px rgba(0, 0, 0, 0.2), inset 0px 0px 0px 2px var(--theme-500);
  background-color: var(--theme-400);
  -moz-box-shadow: 0 1px 0px rgba(0, 0, 0, 0.2), 0 0 0 2px var(--theme-500) inset;
  -webkit-box-shadow: 0 1px 0px rgba(0, 0, 0, 0.2), 0 0 0 2px var(--theme-500) inset;
  -moz-border-radius: 3px;
  -webkit-border-radius: 3px;
  text-shadow: 0 1px 0 var(--theme-500);
}

kbd.active {
  @apply font-bold text-black bg-accent;
  border: 1px solid var(--accent-color);
  text-shadow: 0 1px 0 #e1cbff;
  box-shadow: 0px 1px 0px rgba(0, 0, 0, 0.2), inset 0px 0px 0px 2px #e1cbff;
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
