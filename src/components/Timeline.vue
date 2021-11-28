<template>
  <div class="w-full h-full bg-blue-500">
    <canvas ref="timeline" class="w-full h-full" :width="100" :height="100"></canvas>
  </div>
</template>

<script setup lang="ts">
// defineProps<{}>();
import { onMounted, ref } from "vue";

const timeline = ref<HTMLCanvasElement | undefined>();

// TODO: move into separate file when its too big
class TimelineRenderer {
  private ctx: CanvasRenderingContext2D;

  constructor(public output: HTMLCanvasElement) {
    this.ctx = output.getContext("2d")!; // this will never be undefined lmfao unless shit pc
    this.attachResize();
  }

  private debug(...args: any[]) {
    import.meta.env.DEV && console.log(`[Timeline]`, args);
  }

  private attachResize() {
    new ResizeObserver((value) => {
      this.resize();
    }).observe(this.output.parentElement!);
    this.resize();
  }

  public resize() {
    const parent = this.output.parentElement!;
    this.debug("Resizing timeline", parent.clientWidth, parent.clientHeight);
    this.output.width = parent.clientWidth;
    this.output.height = parent.clientHeight;

    // you have to rerender in order to make stuff look correct
    this.rerender();
  }

  public rerender() {
    this.ctx.clearRect(0, 0, this.output.width, this.output.height);
    this.ctx.fillStyle = "red";
    this.ctx.fillRect(0, 0, this.output.width, this.output.height);
  }
}

onMounted(() => {
  const renderer = new TimelineRenderer(timeline.value!);
  renderer.rerender();
});
</script>

<style scoped lang="postcss"></style>
