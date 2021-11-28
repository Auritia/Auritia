<template>
  <div @mousedown="handleMouseDown" ref="resizableDiv" class="resizableDiv h-full">
    <slot />
  </div>
</template>
<script lang="ts" setup>
import { ref } from "vue";

const emit = defineEmits(["collapsed"]);

let mousePosition = 0;

const resizableDiv = ref<HTMLDivElement>();

const handleMouseDown = (e: MouseEvent) => {
  const width = resizableDiv.value!.getBoundingClientRect().width;
  if (width - e.offsetX < 8) {
    mousePosition = e.x;
    document.addEventListener("mousemove", resize, false);
    document.addEventListener(
      "mouseup",
      function () {
        document.removeEventListener("mousemove", resize, false);
        document.removeEventListener("mouseup", resize, false);
      },
      false
    );
  }
};

function resize(e: MouseEvent) {
  if (e.x < 128) return emit("collapsed");
  const dx = e.x - mousePosition;
  mousePosition = e.x;
  resizableDiv.value!.style.width = parseInt(getComputedStyle(resizableDiv.value!, "").width) + dx + "px";
}
</script>

<style lang="postcss" scoped>
.resizableDiv {
  @apply pr-1 bg-theme-200 hover:bg-accent;
  cursor: ew-resize;
}
</style>
