import { throwStatement } from "@babel/types";
import { scopedDebug } from "./Debug";

export class DynamicCanvas {
  private ctx: CanvasRenderingContext2D;
  private debug = scopedDebug("DynamicCanvas");

  constructor(public output: HTMLCanvasElement) {
    this.ctx = output.getContext("2d")!; // this will never be undefined lmfao unless shit pc
    this.ctx.imageSmoothingEnabled = false;
    this.attachResize();
    this.rerender();
  }

  private attachResize() {
    new ResizeObserver(() => this.resize()).observe(this.output.parentElement!);
  }

  public resize() {
    const parent = this.output.parentElement!;
    this.debug("Resizing timeline", parent.clientWidth, parent.clientHeight);
    this.output.width = parent.clientWidth;
    this.output.height = parent.clientHeight;

    // you have to rerender in order to make stuff look correct
    this.rerender();
  }

  public rerender() {
    // implement in subclass
    window.devicePixelRatio = 2;
    this.ctx.clearRect(0, 0, this.output.width, this.output.height);
    this.ctx.fillStyle = "red";
    this.ctx.fillRect(0, 0, this.output.width, this.output.height);
    this.ctx.fillStyle = "white";

    const message = "❌ You didnt implement this dumbass";
    this.ctx.font = "30px Comic Sans MS";
    this.ctx.fillText(message, this.output.width / 2 - this.ctx.measureText(message).width / 2, this.output.height / 2);
  }
}
