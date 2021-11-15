<template>
  <div class="flex text-theme-700">
    <button @click="appWindow.isFullscreen().then((isFullscreen) => appWindow.setFullscreen(!isFullscreen))" class="controlButton hover:bg-theme-400 active:bg-theme-500">
      <i-fluency-fullscreen />
    </button>

    <button @click="appWindow.minimize()" class="controlButton hover:bg-theme-400 active:bg-theme-500">
      <i-fluency-minimize />
    </button>

    <button @click="appWindow.toggleMaximize()" class="controlButton hover:bg-theme-400 active:bg-theme-500">
      <i-fluency-maximize />
    </button>

    <button @click="appWindow.close()" class="controlButton hover:bg-red-500 active:bg-red-400">
      <i-fluency-x />
    </button>
  </div>
</template>
<script lang="ts" setup>
import { appWindow } from "@tauri-apps/api/window";
</script>

<style scoped lang="postcss">
.controlButton {
  @apply hover:text-white h-full flex items-center justify-center py-1.5 px-4 text-sm;
}
</style>
