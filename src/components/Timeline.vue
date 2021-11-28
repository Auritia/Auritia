<template>
  <div class="w-full h-full bg-blue-500">
    <div class="px-2 py-1">
      <input type="range" step="0.1" min="0.1" max="4" v-model="upsampleValue" />
      {{ upsampleValue }}
    </div>
    <canvas ref="timeline" class="w-full h-full transform origin-top-left"></canvas>
  </div>
</template>

<script setup lang="ts">
// defineProps<{}>();
import { onMounted, Ref, ref, watch } from "vue";
import { DynamicCanvas } from "~/logic/DynamicCanvas";
import { TimelineRenderer } from "~/logic/TimelineRenderer";

let renderer: TimelineRenderer | undefined;
const timeline = ref<HTMLCanvasElement | undefined>();
const upsampleValue = ref(1);

watch(upsampleValue, () => {
  renderer?.changeUpsampling(upsampleValue.value);
});

onMounted(() => {
  renderer = new TimelineRenderer(timeline.value!);
  renderer.draw();
});
</script>

<style scoped lang="postcss"></style>
