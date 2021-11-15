<template>
  <div class="flex h-full">
    <ul class="text-theme-700 bg-theme-100 text-2xl h-full">
      <li @click="switchExplorerView('samples')" :class="isAtSamples && 'active'" class="tab"><i-fluency-file /></li>
      <li @click="switchExplorerView('plugins')" :class="isAtPlugins && 'active'" class="tab"><i-fluency-plugin /></li>
    </ul>

    <ul v-if="isAtSamples" class="text-theme-700 p-1 bg-theme-200 scrollable overflow-scroll text-xs h-full w-64">
      <li class="hover:text-accent cursor-pointer flex items-center gap-1" v-for="file of files" :Lkey="file.path"><i-fluency-file />{{ file.name }}</li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { fs } from "@tauri-apps/api";
import { FileEntry } from "@tauri-apps/api/fs";
import { computed, Ref, ref } from "vue";
import { useRoute, useRouter } from "vue-router";
const route = useRoute();
const router = useRouter();
const activeRoute = computed(() => route.params.explorer);
const isAtSamples = computed(() => activeRoute.value === "samples");
const isAtPlugins = computed(() => activeRoute.value === "plugins");
const switchExplorerView = (name: string) => (name === activeRoute.value ? router.push({ name: "DAW" }) : router.push({ name: "DAW", params: { explorer: name } }));

const files: Ref<FileEntry[]> = ref([]);

fs.readDir("D:/New Geoxor Projects/04. kiss Project").then((content) => (files.value = content));
</script>

<style scoped lang="postcss">
.tab {
  @apply hover:text-accent p-2 border-transparent border-3 flex items-center cursor-pointer;
  &.active {
    @apply border-l-accent;
  }
}
</style>
