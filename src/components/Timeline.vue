<template>
  <div class="w-full h-full bg-theme-100">
    <div class="flex items-center gap-2 text-xs px-2 py-1">
      <input type="range" step="0.1" min="0.1" max="4" v-model="upsampleValue" />
      <span>Upsample: {{ upsampleValue }}</span>
      <input type="range" step="0.001" :min="MINIMAL_V_ZOOM" :max="MAXIMAL_V_ZOOM" v-model="verticalZoomLevel" />
      <span>Vertical Zoom: {{ verticalZoomLevel }}</span>
      <div class="flex-1"></div>
      <Button @click="addTrack">Add Track</Button>
      <Button @click="deleteTrack">Delete Track</Button>
    </div>
    <canvas ref="timeline" @wheel="handleWheel" class="w-full h-full transform origin-top-left"></canvas>
  </div>
</template>

<script setup lang="ts">
import { onKeyStroke, useKeyModifier } from "@vueuse/core";
import { onMounted, ref, watch } from "vue";
import { rootHexColor, minmax } from "~/logic";
import { TimelineRenderer } from "~/logic/TimelineRenderer";
import Button from "./shared/Button.vue";

const crtl = useKeyModifier("Control");
const shift = useKeyModifier("Shift");

let renderer: TimelineRenderer | undefined;

const timeline = ref<HTMLCanvasElement | undefined>();
const upsampleValue = ref(2);
const verticalZoomLevel = ref(1 / 8);

// Zoom constraints
const V_ZOOM_STEP = 1 / 32;
const MINIMAL_V_ZOOM = 1 / 16;
const MAXIMAL_V_ZOOM = 1 / 4;

// Zoom handles
const verticalZoomIn = () =>
  (verticalZoomLevel.value = minmax(+verticalZoomLevel.value + V_ZOOM_STEP, MINIMAL_V_ZOOM, MAXIMAL_V_ZOOM));

const verticalZoomOut = () =>
  (verticalZoomLevel.value = minmax(+verticalZoomLevel.value - V_ZOOM_STEP, MINIMAL_V_ZOOM, MAXIMAL_V_ZOOM));

const handleWheel = (e: WheelEvent) => shift.value && (e.deltaY < 0 ? verticalZoomIn() : verticalZoomOut());

// Track handles
const addTrack = () => renderer!.addTrack();
const deleteTrack = () => renderer!.deleteTrack();

// Rerender on value updates
watch(verticalZoomLevel, () => renderer!.setVerticalZoom(verticalZoomLevel.value));
watch(upsampleValue, () => renderer!.changeUpsampling(upsampleValue.value));

// Subdivision listeners
onKeyStroke("1", () => crtl.value && renderer!.lowerSubBarDivision());
onKeyStroke("2", () => crtl.value && renderer!.raiseSubBarDivision());

onMounted(() => {
  renderer = new TimelineRenderer(timeline.value!, {
    highShade: rootHexColor("--theme-300"),
    lowShade: rootHexColor("--theme-250"),
    gridColor: rootHexColor("--theme-100"),
  });

  renderer.draw();
});
</script>

<style scoped lang="postcss"></style>
