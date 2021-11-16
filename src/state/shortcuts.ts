import { onKeyStroke, useKeyModifier } from "@vueuse/core";
import { appWindow } from "@tauri-apps/api/window";
import router from "~/router";
import { useState } from ".";

const state = useState();

const shift = useKeyModifier("Shift");
const crtl = useKeyModifier("Control");
const alt = useKeyModifier("Alt");

// F11 -> Fullscreen
onKeyStroke("F11", () => appWindow.isFullscreen().then((value) => appWindow.setFullscreen(!value)));

// F1 -> Open Samples Picker
onKeyStroke("F1", () => router.push({ name: "DAW", params: { explorer: "samples" } }));

// F2 -> Open Plugins Picker
onKeyStroke("F2", () => router.push({ name: "DAW", params: { explorer: "plugins" } }));

// Alt + Enter -> Toggle Maximize
onKeyStroke("Enter", () => alt.value && appWindow.toggleMaximize());

// Ctrl + M -> Toggle Metronome
onKeyStroke("m", () => crtl.value && state.toggleMetronome());

// Ctrl + L -> Toggle Metronome
onKeyStroke("l", () => crtl.value && state.toggleLoop());

// Ctrl + N -> New Project
onKeyStroke("n", () => crtl.value && state.reactive.project.new());

// Ctrl + O -> Open Project
onKeyStroke("o", () => crtl.value && state.reactive.project.open());

// Ctrl + S -> Save Project
onKeyStroke("s", () => crtl.value && state.reactive.project.save());

// Ctrl + Alt + S -> Save Project As
onKeyStroke("s", () => crtl.value && alt.value && state.reactive.project.saveAs());
