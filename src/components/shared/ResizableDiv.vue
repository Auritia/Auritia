<template>
  <div @mousedown="handleMouseDown" ref="resizableDiv" class="resizableDiv h-full">
    <slot />
  </div>
</template>
<script lang="ts" setup>
import { ref } from "vue";

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
  const dx = e.x - mousePosition;
  mousePosition = e.x;
  resizableDiv.value!.style.width = parseInt(getComputedStyle(resizableDiv.value!, "").width) + dx + "px";
}
</script>

<style scoped>
.resizableDiv {
  @apply pr-1;
  cursor: ew-resize;
}
</style>
