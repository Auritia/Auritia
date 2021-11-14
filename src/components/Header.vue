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
        class="controlButton"
      >
        <i-fluency-fullscreen />
      </button>

      <button @click="appWindow.minimize()" class="controlButton">
        <i-fluency-minimize />
      </button>

      <button @click="appWindow.toggleMaximize()" class="controlButton">
        <i-fluency-maximize />
      </button>

      <button
        @click="appWindow.close()"
        class="controlButton hover:bg-red-500 active:bg-red-400"
      >
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
  @apply hover:bg-theme-400 active:bg-theme-500 h-full flex items-center justify-center py-1.5 px-4 text-sm text-white;
}
</style>
