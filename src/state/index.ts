import { emit } from "@tauri-apps/api/event";
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
  }

  public toggleMetronome() {
    this.reactive.isMetronomeEnabled = !this.reactive.isMetronomeEnabled;
    emit("set_metronome", JSON.stringify(this.reactive.isMetronomeEnabled));
  }

  public toggleLoop() {
    this.reactive.isLoopEnabled = !this.reactive.isLoopEnabled;
    emit("set_loop", JSON.stringify(this.reactive.isLoopEnabled));
  }

  public play() {
    emit(
      "play",
      JSON.stringify({
        pos: 0,
      })
    );
    this.reactive.isPlaying = true;
  }
  public pause() {
    this.reactive.isPlaying = false;
  }
  public stop() {
    emit("stop");
    this.reactive.isPlaying = false;
    this.reactive.playheadPosition = 0;
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

  public setTempo(value: number) {
    this.tempo = value;
    emit("set_bpm", JSON.stringify(this.tempo));
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
  isPlaying: boolean;
  playheadPosition: number;
}

const globalState = new State({
  project: new Project(),
  isMetronomeEnabled: false,
  isLoopEnabled: false,
  playheadPosition: 0,
  isPlaying: false,
});

export const useState = () => globalState;
