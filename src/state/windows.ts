import { WebviewWindow } from "@tauri-apps/api/window";

export const openPreferences = () => {
  const preferencesWindow = new WebviewWindow("Preferences", {
    url: "/preferences",
    decorations: false,
    width: 500,
    height: 700,
    center: true,
  });
};
