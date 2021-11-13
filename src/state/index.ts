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

listen("open", async (event) => {
  console.log("open");
  const files = await open(AURITIA_FILE_FILTER);
  console.log(files);
});
listen("save", async (event) => {
  console.log("save");
  const savePath = await save(AURITIA_FILE_FILTER);
  console.log(savePath);
});
listen("save_as", async (event) => {
  console.log("save_as");
  const savePath = await save(AURITIA_FILE_FILTER);
  console.log(savePath);
});
listen("render", async (event) => {
  console.log("render");
});
listen("project_info", async (event) => {
  console.log("project_info");
});
listen("preferences", async (event) => {
  console.log("preferences");
});
listen("docs", async (event) => {
  console.log("docs");
});
