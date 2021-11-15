<template>
  <div data-tauri-drag-region class="bg-theme-100 text-xs flex text-light-50 justify-between items-center">
    <div class="flex gap-4 h-full items-center">
      <img src="../assets/horizontal_logo.svg" class="ml-2" alt="" />

      <Menu title="File">
        <MenuOption title="Open..." @click="state.project.open" />
        <MenuOption title="Save Project" @click="state.project.save" />
        <MenuOption title="Save Project As..." @click="state.project.saveAs" />
        <MenuSplitter />
        <MenuOption title="Render Audio..." @click="state.project.render" />
        <MenuOption title="Project Info..." @click="state.project.info" />
      </Menu>

      <Menu title="Settings">
        <MenuOption title="Preferences..." @click="" />
      </Menu>

      <Menu title="Help">
        <MenuOption title="Documentation..." @click="" />
      </Menu>
    </div>

    <p>v{{ version }} | {{ state.project.name }}</p>
    <!-- <div class="text-sm flex gap-1">
      <i-fluency-play @click="play()" :class="state.isPlaying && 'text-green-400'" class="clicky" />
      <i-fluency-stop @click="stop()" class="clicky" />
      <i-fluency-pause @click="pause()" class="clicky" />
    </div> -->
    <ControlButtons />
  </div>
</template>

<script lang="ts" setup>
import { useState } from "~/state";
import Menu from "./shared/Menu.vue";
import MenuOption from "./shared/MenuOption.vue";
import MenuSplitter from "./shared/MenuSplitter.vue";
import ControlButtons from "./ControlButtons.vue";
const { state } = useState();
const version = import.meta.env.PACKAGE_VERSION;
</script>

<style lang="postcss">
select::-ms-expand {
  display: none;
}
</style>
