<template>
  <div data-tauri-drag-region class="flex justify-between gap-2 text-xs p-1">
    <!-- {{ state.project.tempo }} -->
    <!-- {{ state.project.timeSignature }} -->
    <div class="flex gap-0.5 text-theme-900">
      <Slider v-model="state.project.tempo" :min="10" :max="999" :step="1.0" :percent="false" :default="128" />
      <Button @click="state.isMetronomeEnabled = !state.isMetronomeEnabled" :isActive="state.isMetronomeEnabled">
        <i-fluency-metronome />
      </Button>
      <Button @click="state.isLoopEnabled = !state.isLoopEnabled" :isActive="state.isLoopEnabled">
        <i-fluency-loop />
      </Button>
    </div>

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
import Slider from "./shared/Slider.vue";

const { state, play, stop, pause } = useState();
</script>
