import { emit, listen } from "@tauri-apps/api/event";
import { open, save } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { AURITIA_FILE_FILTER } from "~/constants";
import { reactive } from "vue";

const preferences = () => {
  console.log("preferences");
};
const docs = () => {
  console.log("docs");
};

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

/**
 * A Auritia Project file
 */
export class Project {
  name: string;

  constructor(name: string = "New Project") {
    this.name = name;
  }

  /**
   * Opens a project
   */
  public async open() {
    const files = await open(AURITIA_FILE_FILTER);
    console.log(files);
  }
  /**
   * Saves the current project
   */
  public async save() {
    const savePath = await save(AURITIA_FILE_FILTER);
    console.log(savePath);
  }
  /**
   * Saves the current project as a new file
   */
  public async saveAs() {
    const savePath = await save(AURITIA_FILE_FILTER);
    console.log(savePath);
  }
  /**
   * Renders the project
   */
  public render() {
    console.log("render");
  }
  /**
   * Shows project info
   */
  public info() {
    console.log("project_info");
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
