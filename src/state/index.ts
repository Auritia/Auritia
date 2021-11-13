import { emit, listen } from "@tauri-apps/api/event";
import { open, save } from "@tauri-apps/api/dialog";
// With the Tauri API npm package:
import { invoke } from "@tauri-apps/api/tauri";

// Invoke the command
invoke("my_custom_command");
import { AURITIA_FILE_FILTER } from "~/constants";
import { reactive } from "vue";

listen("open", async (event) => {
  const files = await open(AURITIA_FILE_FILTER);
  console.log(files);
});
listen("save", async (event) => {
  const savePath = await save(AURITIA_FILE_FILTER);
  console.log(savePath);
});
listen("save_as", async (event) => {
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

/**
 * Le abstract class that holds the attribute state
 * @author Goxer & N1kO23
 */
export abstract class Store<T extends Object> {
  public state: T;

  public constructor(data: T) {
    this.state = reactive(data) as T;
  }
}

export type AppEvents = "stopSound";
export class State extends Store<IAppState> {
  constructor(state: IAppState) {
    super(state);
    window.addEventListener("keydown", (e: KeyboardEvent) => {});
  }

  public play() {
    this.state.isPlaying = true;
    invoke("beep");

    console.log("now playing");

    emit(
      "play",
      JSON.stringify({
        pos: 0,
      })
    );
  }
  public pause() {
    this.state.isPlaying = false;
  }
  public stop() {
    this.state.isPlaying = false;
    this.state.playheadPosition = 0;
  }
}

export class Project {
  name: string;

  constructor(name: string = "New Project") {
    this.name = name;
  }
}

export type TimeSignature = [number, number];

export interface IAppState {
  project: Project;
  isMetronomeEnabled: boolean;
  tempo: number;
  isPlaying: boolean;
  playheadPosition: number;
  timeSignature: TimeSignature;
}

const globalState = new State({
  project: new Project(),
  isMetronomeEnabled: false,
  playheadPosition: 0,
  tempo: 128,
  isPlaying: false,
  timeSignature: [4, 4],
});

export const useState = () => globalState;
