import { throwStatement } from "@babel/types";
import { scopedDebug } from "./Debug";

export class DynamicCanvas {
  private ctx: CanvasRenderingContext2D;
  private debug = scopedDebug("DynamicCanvas");

  /**
   * This propertyu multiplies the render scale by an amount to reduce the bluriness canvases have.
   *
   * 1x is the default which is very blurry and looks like shit.
   * 2x looks a lot sharper and normal.
   * 3x and above looks too sharp as if theres no anti-aliasing.
   *
   * It's also possible to set it to numbers less than 1x to achive lower resolutions
   * kinda like how video games have render resolution options like 0.8 etc
   */
  private upsampling = 2;

  constructor(public output: HTMLCanvasElement) {
    this.ctx = output.getContext("2d")!; // this will never be undefined lmfao unless shit pc
    this.ctx.imageSmoothingEnabled = false;
    this.attachResize();
    this.draw();
  }

  public changeUpsampling(upsampling: number) {
    this.upsampling = upsampling;
    this.resize(); // must resize to cause the canvas to apply upsampling change.
  }

  private attachResize() {
    new ResizeObserver(() => this.resize()).observe(this.output.parentElement!);
  }

  public resize() {
    const parent = this.output.parentElement!;
    this.debug("Resizing timeline", parent.clientWidth, parent.clientHeight);
    this.output.width = parent.clientWidth * this.upsampling;
    this.output.height = parent.clientHeight * this.upsampling;

    // you have to draw in order to make stuff look correct
    this.draw();
  }

  public draw() {
    // implement in subclass
    this.ctx.clearRect(0, 0, this.output.width, this.output.height);
    this.ctx.fillStyle = "red";
    this.ctx.fillRect(0, 0, this.output.width, this.output.height);
    this.ctx.fillStyle = "white";

    const message = "⚠️ You didnt implement draw() dumbass";
    this.ctx.font = `${50 * this.upsampling}px Arial`;
    this.ctx.fillText(message, this.output.width / 2 - this.ctx.measureText(message).width / 2, this.output.height / 2);
  }
}
