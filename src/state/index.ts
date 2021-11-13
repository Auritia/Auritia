import { emit, listen } from "@tauri-apps/api/event";
import { open, save } from "@tauri-apps/api/dialog";

const AURITIA_FILE_FILTER = {
  filters: [
    {
      name: "Auritia Project",
      extensions: ["aur"],
    },
  ],
};

await listen("open", async (event) => {
  console.log("open");
  const files = await open(AURITIA_FILE_FILTER);
  console.log(files);
});
await listen("save", async (event) => {
  console.log("save");
  const savePath = await save(AURITIA_FILE_FILTER);
  console.log(savePath);
});
await listen("save_as", async (event) => {
  console.log("save_as");
  const savePath = await save(AURITIA_FILE_FILTER);
  console.log(savePath);
});
await listen("render", async (event) => {
  console.log("render");
});
await listen("project_info", async (event) => {
  console.log("project_info");
});
await listen("preferences", async (event) => {
  console.log("preferences");
});
await listen("docs", async (event) => {
  console.log("docs");
});
