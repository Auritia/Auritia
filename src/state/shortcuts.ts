import { onKeyStroke } from "@vueuse/core";
import { appWindow } from "@tauri-apps/api/window";

onKeyStroke("F11", () => appWindow.isFullscreen().then((value) => appWindow.setFullscreen(!value)));
