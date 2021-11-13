<script setup lang="ts">
import Header from "./components/Header.vue";
</script>

<template>
  <Header />
  <route-view />
</template>

<style lang="postcss">
#app,
body,
html {
  @apply h-full w-full;
}
</style>
