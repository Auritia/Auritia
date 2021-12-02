<template>
  <div data-tauri-drag-region class="bg-theme-100 text-xs flex justify-between items-center">
    <div class="flex gap-4 h-full items-center">
      <img src="../assets/horizontal_logo.svg" class="ml-2 h-4" alt="" />

      <Menu title="File">
        <MenuOption
          hint="Open an Auritia project file"
          :shortcuts="['CTRL', 'O']"
          title="Open..."
          @click="reactive.project.open"
        />
        <MenuOption
          hint="Save the current project"
          :shortcuts="['CTRL', 'S']"
          title="Save Project"
          @click="reactive.project.save"
        />
        <MenuOption
          hint="Save the current project to a new file"
          :shortcuts="['CTRL', 'SHIFT', 'S']"
          title="Save Project As..."
          @click="reactive.project.saveAs"
        />
        <MenuSplitter />
        <MenuOption
          hint="Render the selected region to a file"
          :shortcuts="['CTRL', 'R']"
          title="Render Audio..."
          @click="reactive.project.render"
        />
        <MenuOption
          hint="View information about the current project"
          :shortcuts="['CTRL', 'I']"
          title="Project Info..."
          @click="reactive.project.info"
        />
      </Menu>

      <Menu title="Settings">
        <MenuOption
          hint="Opens the settings menu"
          :shortcuts="['CTRL', ',']"
          title="Preferences..."
          @click="openPreferences()"
        />
      </Menu>

      <Menu title="Help">
        <MenuOption hint="Opens the Auritia documentation on the internet" title="Documentation..." @click="" />
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
