<template>
  <div
    data-tauri-drag-region
    class="
      bg-theme-100
      text-xs
      flex
      text-light-50
      justify-between
      items-center
      font-semibold
    "
  >
    {{ state.tempo }}
    {{ state.isMetronomeEnabled }}
    {{ state.timeSignature[0] }} / {{ state.timeSignature[1] }}
    <div class="text-sm flex gap-1">
      <i-fluency-play
        @click="play()"
        :class="state.isPlaying && 'text-green-400'"
        class="clicky"
      />
      <i-fluency-stop @click="stop()" class="clicky" />
      <i-fluency-pause @click="pause()" class="clicky" />
    </div>
    <div class="flex">
      <button
        @click="
          appWindow
            .isFullscreen()
            .then((isFullscreen) => appWindow.setFullscreen(!isFullscreen))
        "
        class="controlButton hover:bg-theme-400"
      >
        <i-fluency-fullscreen />
      </button>

      <button
        @click="appWindow.minimize()"
        class="controlButton hover:bg-theme-400"
      >
        <i-fluency-minimize />
      </button>

      <button
        @click="appWindow.toggleMaximize()"
        class="controlButton hover:bg-theme-400"
      >
        <i-fluency-maximize />
      </button>

      <button @click="appWindow.close()" class="controlButton hover:bg-red-500">
        <i-fluency-x />
      </button>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { useState } from "~/state";
import { appWindow } from "@tauri-apps/api/window";
const { state, play, stop, pause } = useState();
</script>

<style lang="postcss">
.controlButton {
  @apply h-full flex items-center justify-center py-1.5 px-4 text-sm text-white;
}
</style>
