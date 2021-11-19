<template>
  <div data-tauri-drag-region class="bg-theme-100 text-xs flex text-theme-700 justify-between items-center">
    <div class="flex gap-4 h-full items-center">
      <img src="../assets/horizontal_logo.svg" class="ml-2" alt="" />

      <Menu title="File">
        <MenuOption title="Open..." @click="reactive.project.open" />
        <MenuOption title="Save Project" @click="reactive.project.save" />
        <MenuOption title="Save Project As..." @click="reactive.project.saveAs" />
        <MenuSplitter />
        <MenuOption title="Render Audio..." @click="reactive.project.render" />
        <MenuOption title="Project Info..." @click="reactive.project.info" />
      </Menu>

      <Menu title="Settings">
        <MenuOption title="Preferences..." @click="openPreferences()" />
      </Menu>

      <Menu title="Help">
        <MenuOption title="Documentation..." @click="" />
      </Menu>
    </div>

    <p>v{{ version }} | {{ reactive.project.name }}</p>

    <ControlButtons />
  </div>
</template>

<script lang="ts" setup>
import { useState } from "~/state";
import Menu from "./shared/Menu.vue";
import MenuOption from "./shared/MenuOption.vue";
import MenuSplitter from "./shared/MenuSplitter.vue";
import ControlButtons from "./ControlButtons.vue";
import { openPreferences } from "~/state/windows";
const { reactive } = useState();
const version = import.meta.env.PACKAGE_VERSION;
</script>

<style lang="postcss">
select::-ms-expand {
  display: none;
}
</style>
