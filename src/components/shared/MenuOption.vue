<template>
  <button class="hover:text-accent hover:bg-theme-200 py-1 px-6 w-full flex justify-between items-center">
    <p>{{ title }}</p>
    <div class="text-10px flex gap-1">
      <kbd v-for="(shortcut, i) of shortcuts">{{ shortcut }}</kbd>
    </div>
  </button>
</template>

<script lang="ts" setup>
defineProps<{
  title: string;
  shortcuts?: string[];
}>();
</script>
