import { emit, listen } from "@tauri-apps/api/event";
import { open, save } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { AURITIA_FILE_FILTER } from "~/constants";
import { reactive } from "vue";

/**
 * Le abstract class that holds the attribute state
 * @author Goxer & N1kO23
 */
export abstract class Store<T extends Object> {
  public reactive: T;

  public constructor(data: T) {
    this.reactive = reactive(data) as T;
  }
}

export class State extends Store<IAppState> {
  constructor(state: IAppState) {
    super(state);
    window.addEventListener("keydown", (e: KeyboardEvent) => {});

    listen<string>("error", (event) => (this.reactive.errorText = event.payload));
  }

  public setHint(hint: string) {
    this.reactive.hint = hint;
  }

  public async toggleMetronome() {
    this.reactive.isMetronomeEnabled = await invoke<boolean>("set_metronome", { value: !this.reactive.isMetronomeEnabled });
  }

  public async toggleLoopPreview() {
    this.reactive.isLoopPreviewEnabled = await invoke<boolean>("set_loop_preview", {
      value: !this.reactive.isLoopPreviewEnabled,
    });
  }

  public tapMetronome() {
    // emit("tap_metronome");
  }

  public toggleLoop() {
    this.reactive.isLoopEnabled = !this.reactive.isLoopEnabled;
    // emit("set_loop", JSON.stringify(this.reactive.isLoopEnabled));
  }

  public async play() {
    await invoke<boolean>("play");
    this.reactive.isPlaying = true;
  }
  public async pause() {
    this.reactive.isPlaying = false;
  }
  public async stop() {
    await invoke<boolean>("stop");
    this.reactive.isPlaying = false;
  }
}

/**
 * A Auritia Project file
 */
export class Project {
  name: string;
  tempo: number;
  timeSignature: TimeSignature;

  constructor(name: string = "New Project", tempo: number = 120, timeSignature: TimeSignature = [4, 4]) {
    this.name = name;
    this.tempo = tempo;
    this.timeSignature = timeSignature;
  }

  public async setTempo(value: number) {
    if (this.tempo !== value) {
      this.tempo = await invoke<number>("set_bpm", { value: value });
    }
  }

  /**
   * Creates a new project
   */
  public async new() {
    this.constructor();
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
  isLoopEnabled: boolean;
  isLoopPreviewEnabled: boolean;
  hint?: string;
  errorText: string | undefined;
  isPlaying: boolean;
  playheadPosition: number;
}

const globalState = new State({
  project: new Project(),
  isMetronomeEnabled: false,
  isLoopEnabled: false,
  isLoopPreviewEnabled: false,
  playheadPosition: 0,
  errorText: undefined,
  isPlaying: false,
});

export const useState = () => globalState;
