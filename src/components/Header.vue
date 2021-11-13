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
        class="
          h-full
          flex
          items-center
          justify-center
          hover:bg-theme-200
          py-1
          px-4
          text-base text-white
        "
      >
        <i-fluency-minimize @click="appWindow.minimize()" />
      </button>
      <button
        class="
          h-full
          flex
          items-center
          justify-center
          hover:bg-theme-200
          py-1
          px-4
          text-base text-white
        "
      >
        <i-fluency-maximize @click="appWindow.toggleMaximize()" />
      </button>
      <button
        class="
          h-full
          flex
          items-center
          justify-center
          hover:bg-red-500
          py-1
          px-4
          text-base text-white
        "
      >
        <i-fluency-x @click="appWindow.close()" />
      </button>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { useState } from "~/state";
import { appWindow } from "@tauri-apps/api/window";
const { state, play, stop, pause } = useState();
</script>
