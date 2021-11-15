<template>
  <div data-tauri-drag-region class="flex justify-between gap-2 text-xs p-1">
    <!-- {{ state.project.tempo }} -->
    <!-- {{ state.project.timeSignature }} -->
    <Button @click="toggleMetronome()" :isActive="state.isMetronomeEnabled">
      <i-fluency-metronome />
    </Button>
    <div class="text-sm flex gap-1">
      <i-fluency-play @click="play()" :class="state.isPlaying && 'text-green-400'" class="clicky" />
      <i-fluency-stop @click="stop()" class="clicky" />
      <i-fluency-pause @click="pause()" class="clicky" />
    </div>
    <div></div>
  </div>
</template>

<script setup lang="ts">
import { useState } from "~/state";
import Button from "./shared/Button.vue";

const { state, play, stop, pause, toggleMetronome } = useState();
</script>
