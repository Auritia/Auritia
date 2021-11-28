<template>
  <div class="w-full h-full bg-theme-100">
    <div class="flex items-center gap-2 text-xs px-2 py-1">
      <input type="range" step="0.1" min="0.1" max="4" v-model="upsampleValue" />
      <span>Upsample: {{ upsampleValue }}</span>
      <input type="range" step="0.01" :min="1 / 16" :max="1 / 4" v-model="verticalZoomLevel" />
      <span>Vertical Zoom: {{ verticalZoomLevel }}</span>
      <div class="flex-1"></div>
      <Button @click="addTrack">Add Track</Button>
      <Button @click="deleteTrack">Delete Track</Button>
    </div>
    <canvas ref="timeline" class="w-full h-full transform origin-top-left"></canvas>
  </div>
</template>

<script setup lang="ts">
// defineProps<{}>();
import { onMounted, ref, watch } from "vue";
import { DynamicCanvas } from "~/logic/DynamicCanvas";
import { TimelineRenderer } from "~/logic/TimelineRenderer";
import Button from "./shared/Button.vue";

let renderer: TimelineRenderer | undefined;

const timeline = ref<HTMLCanvasElement | undefined>();
const upsampleValue = ref(2);
const verticalZoomLevel = ref(1 / 8);

const addTrack = () => renderer?.addTrack();

const deleteTrack = () => renderer?.deleteTrack();

watch(verticalZoomLevel, () => renderer?.setVerticalZoom(verticalZoomLevel.value));

watch(upsampleValue, () => {
  renderer?.changeUpsampling(upsampleValue.value);
});

onMounted(() => {
  const styles = getComputedStyle(document.documentElement);
  renderer = new TimelineRenderer(timeline.value!, {
    highShade: styles.getPropertyValue("--theme-200").trim(),
    lowShade: styles.getPropertyValue("--theme-300").trim(),
    gridColor: styles.getPropertyValue("--theme-100").trim(),
  });
  renderer.draw();
});
</script>

<style scoped lang="postcss"></style>
