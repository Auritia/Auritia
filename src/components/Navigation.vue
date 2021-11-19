<template>
  <div class="flex h-full">
    <ul class="bg-theme-100 text-2xl h-full">
      <li @click="switchExplorerView('samples')" :class="isAtSamples && 'active'" class="tab"><i-fluency-file /></li>
      <li @click="switchExplorerView('plugins')" :class="isAtPlugins && 'active'" class="tab"><i-fluency-plugin /></li>
    </ul>

    <ResizableDiv @collapsed="switchExplorerView" v-if="isAtSamples" class="w-64">
      <ul
        @drop="handleDrop"
        @dragover.prevent
        class="cursor-default p-1 bg-theme-200 scrollable overflow-scroll text-xs h-full"
      >
        <li class="hover:text-accent cursor-pointer flex items-center gap-1" v-for="file of files" :Lkey="file.path">
          <p class="overflow-hidden whitespace-nowrap overflow-ellipsis max-w-full"><i-fluency-file />{{ file.name }}</p>
        </li>
      </ul>
    </ResizableDiv>
  </div>
</template>

<script setup lang="ts">
import { fs } from "@tauri-apps/api";
import { FileEntry } from "@tauri-apps/api/fs";
import { computed, Ref, ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import ResizableDiv from "./shared/ResizableDiv.vue";
const route = useRoute();
const router = useRouter();
const activeRoute = computed(() => route.params.explorer);
const isAtSamples = computed(() => activeRoute.value === "samples");
const isAtPlugins = computed(() => activeRoute.value === "plugins");
const switchExplorerView = (name: string) =>
  name === activeRoute.value ? router.push({ name: "DAW" }) : router.push({ name: "DAW", params: { explorer: name } });

const files: Ref<FileEntry[]> = ref([]);

const handleDrop = (e: DragEvent) => {
  console.log(e);
};

fs.readDir("C:/Users/Geoxor/Documents/VSTs").then((content) => (files.value = content));
</script>

<style scoped lang="postcss">
.tab {
  @apply hover:text-theme-900 p-2 border-transparent border-3 flex items-center cursor-pointer;
  &.active {
    @apply border-l-accent text-accent;
  }
}
</style>
