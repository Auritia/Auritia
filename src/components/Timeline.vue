<template>
  <div class="w-full h-full bg-blue-500">
    <canvas ref="timeline" class="w-full h-full" :width="100" :height="100"></canvas>
  </div>
</template>

<script setup lang="ts">
// defineProps<{}>();
import { onMounted, ref } from "vue";
import { DynamicCanvas } from "~/logic/DynamicCanvas";
import { TimelineRenderer } from "~/logic/TimelineRenderer";

const timeline = ref<HTMLCanvasElement | undefined>();

onMounted(() => {
  const renderer = new TimelineRenderer(timeline.value!);
  renderer.rerender();
});
</script>

<style scoped lang="postcss"></style>
