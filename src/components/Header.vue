<template>
  <div data-tauri-drag-region class="flex justify-between gap-2 text-xs p-1">
    <!-- {{ reactive.project.tempo }} -->
    <!-- {{ reactive.project.timeSignature }} -->
    <div class="flex gap-0.5 text-theme-900">
      <Button hint="Tap to sync the metronome to your speed" @click="state.tapMetronome()">
        <i-fluency-tap />
      </Button>
      <Slider v-model="state.reactive.project.tempo" :min="1" :max="9999" :step="1.0" :percent="false" :default="128" />
      <Button hint="Toggles the metronome" @click="state.toggleMetronome()" :isActive="state.reactive.isMetronomeEnabled">
        <i-fluency-metronome />
      </Button>
      <Button hint="Loops the selected region" @click="state.toggleLoop()" :isActive="state.reactive.isLoopEnabled">
        <i-fluency-loop />
      </Button>
      <Button
        hint="Enables auto loop when previewing short samples"
        @click="state.toggleLoopPreview()"
        :isActive="state.reactive.isLoopPreviewEnabled"
      >
        <i-fluency-infinateloop />
      </Button>
    </div>

    <div class="text-sm flex gap-1">
      <i-fluency-play @click="state.play()" :class="state.reactive.isPlaying && 'text-green-400'" class="clicky" />
      <i-fluency-stop @click="state.stop()" class="clicky" />
      <i-fluency-pause @click="state.pause()" class="clicky" />
    </div>
    <div></div>
  </div>
</template>

<script setup lang="ts">
import { useState } from "~/state";
import Button from "./shared/Button.vue";
import Slider from "./shared/Slider.vue";

const state = useState();
</script>
