<template>
  <button class="hover:text-accent hover:bg-theme-200 py-1 px-8 w-full text-left">{{ title }}</button>
</template>

<script lang="ts" setup>
defineProps<{
  title: string;
}>();
</script>
