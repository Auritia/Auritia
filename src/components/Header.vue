<template>
  <div data-tauri-drag-region class="bg-theme-100 text-xs flex text-light-50 justify-between items-center font-semibold">
    <div class="flex gap-4 h-full items-center">
      <img src="../assets/horizontal_logo.svg" class="ml-2" alt="" />

      <Menu title="File">
        <MenuOption title="Open..." @click="state.project.open" />
        <MenuOption title="Save Project" @click="state.project.save" />
        <MenuOption title="Save Project As..." @click="state.project.saveAs" />
        <MenuSplitter />
        <MenuOption title="Render Audio..." @click="state.project.render" />
        <MenuOption title="Project Info..." @click="state.project.info" />
      </Menu>

      <Menu title="Settings">
        <MenuOption title="Preferences..." @click="" />
      </Menu>

      <Menu title="Help">
        <MenuOption title="Documentation..." @click="" />
      </Menu>

      <!-- <p>v{{ version }} | {{ state.project.name }}</p> -->
    </div>

    <!-- <div class="text-sm flex gap-1">
      <i-fluency-play @click="play()" :class="state.isPlaying && 'text-green-400'" class="clicky" />
      <i-fluency-stop @click="stop()" class="clicky" />
      <i-fluency-pause @click="pause()" class="clicky" />
    </div> -->
    <div class="flex text-theme-700">
      <button @click="appWindow.isFullscreen().then((isFullscreen) => appWindow.setFullscreen(!isFullscreen))" class="controlButton">
        <i-fluency-fullscreen />
      </button>

      <button @click="appWindow.minimize()" class="controlButton">
        <i-fluency-minimize />
      </button>

      <button @click="appWindow.toggleMaximize()" class="controlButton">
        <i-fluency-maximize />
      </button>

      <button @click="appWindow.close()" class="controlButton hover:bg-red-500 active:bg-red-400">
        <i-fluency-x />
      </button>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { useState } from "~/state";
import { appWindow } from "@tauri-apps/api/window";
import Menu from "./shared/Menu.vue";
import MenuOption from "./shared/MenuOption.vue";
import MenuSplitter from "./shared/MenuSplitter.vue";
const { state } = useState();
const version = import.meta.env.PACKAGE_VERSION;
</script>

<style lang="postcss">
.controlButton {
  @apply hover:bg-theme-400 hover:text-white active:bg-theme-500 h-full flex items-center justify-center py-1.5 px-4 text-sm;
}

select::-ms-expand {
  display: none;
}
</style>
