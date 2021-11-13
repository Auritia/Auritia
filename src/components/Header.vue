<template>
  <div class="text-xs font-semibold p-1">
    {{ tempo }}
    {{ isMetronomeEnabled }}
    {{ timeSignature[0] }} / {{ timeSignature[1] }}
  </div>
</template>

<script lang="ts" setup>
import { useState } from "~/state";
const { state, play, pause, stop } = useState();
const { tempo, isMetronomeEnabled, timeSignature } = state;
</script>
