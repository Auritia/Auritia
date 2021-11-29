import { throwStatement } from "@babel/types";
import { scopedDebug } from "./Debug";

/**
 * A canvas that can automatically resize based on it's parent div's size
 * @author Geoxor & Bluskript
 */
export class DynamicCanvas {
  protected ctx: CanvasRenderingContext2D;
  protected debug = scopedDebug("DynamicCanvas");

  /**
   * This property multiplies the render scale by an amount to reduce the bluriness canvases have.
   *
   * 1x is the default which is very blurry and looks like shit.
   * 2x looks a lot sharper and normal.
   * 3x and above looks too sharp as if theres no anti-aliasing.
   *
   * It's also possible to set it to numbers less than 1x to achive lower resolutions
   * kinda like how video games have render resolution options like 0.8 etc
   */
  protected upsampling = 2;

  constructor(public canvas: HTMLCanvasElement) {
    this.ctx = canvas.getContext("2d")!; // this will never be undefined lmfao unless shit pc
    this.ctx.imageSmoothingEnabled = false;
    this.attachResize();
  }

  /**
   * Change upsampling value
   * @param upsampling The new upsample value
   */
  public changeUpsampling(upsampling: number) {
    this.upsampling = upsampling;
    this.resize(); // must resize to cause the canvas to apply upsampling change.
  }

  /**
   * Attaches a listener to the parent div to observe it for size differances
   */
  private attachResize() {
    new ResizeObserver(() => this.resize()).observe(this.canvas.parentElement!);
  }

  /**
   * Changes the size of the canvas to the current parent div size and rerenders it
   */
  public resize() {
    const parent = this.canvas.parentElement!;
    this.debug("Resizing timeline", parent.clientWidth, parent.clientHeight);
    this.canvas.width = parent.clientWidth * this.upsampling;
    this.canvas.height = parent.clientHeight * this.upsampling;

    // you have to draw in order to make stuff look correct
    this.draw();
  }

  /**
   * Automatically scales the desired px for the current upsampling
   * @param px The desired px
   * @returns Upscaled pixel size
   */
  public px(px: number) {
    return px * this.upsampling;
  }

  /**
   * Draw the canvas
   */
  public draw() {
    // implement in subclass
    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
    this.ctx.fillStyle = "red";
    this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
    this.ctx.fillStyle = "white";

    const message = "⚠️ You didnt implement draw() dumbass";
    this.ctx.font = `${this.px(50)}px Arial`;
    this.ctx.fillText(message, this.canvas.width / 2 - this.ctx.measureText(message).width / 2, this.canvas.height / 2);
  }
}
