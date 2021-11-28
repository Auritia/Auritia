import { DynamicCanvas } from "./DynamicCanvas";

export class TimelineRenderer extends DynamicCanvas {
  constructor(public output: HTMLCanvasElement) {
    super(output);
  }
}
