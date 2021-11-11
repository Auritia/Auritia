import { reactive } from "vue";

/**
 * Le abstract class that holds the attribute state
 * @author Goxer & N1kO23
 */
export class Store<T extends Object> {
  public state: T;

  public constructor(data: T) {
    this.state = reactive(data) as T;
  }
}

export type AppEvents = "stopSound";
export class State extends Store<IAppState> {}

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
  timeSignature: TimeSignature;
}

const globalState = new State({
  project: new Project(),
  isMetronomeEnabled: false,
  tempo: 128,
  timeSignature: [4, 4],
});

export const useState = () => globalState;
