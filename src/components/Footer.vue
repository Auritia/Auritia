<template>
  <footer class="flex items-center text-xs w-full h-6">
    <h1 class="flex items-center gap-1 px-2" v-if="text">
      <div class="flex items-center text-xs"><slot /></div>
      {{ text }}
    </h1>
  </footer>
</template>

<script setup lang="ts">
defineProps<{
  text?: string;
}>();
</script>

<style scoped lang="postcss">
footer.active {
  @apply bg-accent font-bold text-black;
}
</style>
