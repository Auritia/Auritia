<script lang="ts" setup>
import { useKeyModifier, useVModel } from "@vueuse/core";
import { computed, ref, watch } from "vue";
import { useState } from "~/state";
const { reactive } = useState();
const props = defineProps<{
  modelValue: number;
  min: number;
  max: number;
  step: number;
  percent: boolean;
  prefix?: string;
  default: number;
}>();
const emit = defineEmits(["update:modelValue"]);
const model = useVModel(props, "modelValue", emit);

const directInputTempo = ref(model.value);
const isDragging = ref(false);
const isGranular = useKeyModifier("Shift");

const directInputField = ref<HTMLInputElement | undefined>();

const directInput = ref(false);

let startY = 0;
let initialValue = 0;

const displayValue = computed(() => {
  if (props.percent) return `${(model.value * 100).toFixed(2)}%`;
  return props.step < 1 ? model.value.toFixed(2) : model.value.toFixed(2);
});

// Sets model to 0 if alt is held. Otherwise sets dragging to true.
const onMouseDown = (e: MouseEvent) => {
  if (e.altKey) {
    reactive.project.setTempo(props.default);
  } else {
    isDragging.value = true;
    startY = e.clientY;
    initialValue = model.value;
  }
};
const step = computed(() => (isGranular.value ? props.step / 10 : props.step));
const roundNearestStep = (value: number) => {
  return Math.ceil(value / step.value) * step.value;
};
const onMove = (e: MouseEvent) => {
  if (!isDragging.value) return;
  const scale = props.max - props.min;
  const distance = startY - e.clientY;
  const stepping = props.max / step.value;
  const val = (distance * scale) / stepping;
  reactive.project.setTempo(Math.min(Math.max(initialValue + roundNearestStep(val), props.min), props.max));
};
const onMouseUp = () => {
  isDragging.value = false;
  startY = model.value;
};
const onDirectInputKey = (event: KeyboardEvent) => {
  if (event.key === "Escape") {
    directInputTempo.value = model.value;
    directInput.value = false;
  }
  if (event.key === "Enter") {
    reactive.project.setTempo(directInputTempo.value);
    directInput.value = false;
  }
};
watch(isDragging, () => {
  if (isDragging.value) {
    document.addEventListener("mousemove", onMove);
    document.addEventListener("mouseup", onMouseUp);
  } else {
    document.removeEventListener("mousemove", onMove);
    document.removeEventListener("mouseup", onMouseUp);
  }
});
const pop = ref(false);
watch(model, () => {
  pop.value = true;
  setTimeout(() => (pop.value = false), 100);
});
</script>

<template>
  <div class="modifier" @dblclick="directInput = true">
    <div
      @mousedown.passive="onMouseDown"
      @mouseup.passive="isDragging = false"
      class="value flex items-center justify-center bg-theme-300 py-0.5 px-1 min-w-12 text-secondary-400"
    >
      <input
        class="bg-theme-300 appearance-none bg-transparentborder-none w-12"
        type="number"
        v-if="directInput"
        @keydown="onDirectInputKey"
        ref="directInputField"
        v-model="directInputTempo"
      />
      <h1 v-else :class="{ pop }">{{ prefix }}{{ displayValue }}</h1>
    </div>
  </div>
</template>

<style lang="postcss" scoped>
.modifier {
  @apply text-11px select-none h-full bg-theme-300 transition transform duration-100 flex overflow-hidden;
  cursor: ns-resize;
  &:active .value {
    @apply text-primary-300;
  }
}
</style>
