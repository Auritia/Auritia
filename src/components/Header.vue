<template>
  <div class="flex gap-2 text-xs p-1">
    <!-- {{ state.project.tempo }} -->
    <!-- {{ state.project.timeSignature }} -->
    <Button @click="toggleMetronome()" :isActive="state.isMetronomeEnabled">
      <i-fluency-metronome />
    </Button>
  </div>
</template>

<script setup lang="ts">
import { useState } from "~/state";
import Button from "./shared/Button.vue";

const { state, toggleMetronome } = useState();
</script>
