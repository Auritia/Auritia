<template>
  <div class="text-xs flex justify-between items-center font-semibold p-1">
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
    <div></div>
  </div>
</template>

<script lang="ts" setup>
import { useState } from "~/state";
const { state, play, stop, pause } = useState();
</script>
