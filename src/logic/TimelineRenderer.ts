import { minmax } from ".";
import { DynamicCanvas } from "./DynamicCanvas";

export interface ColorPalette {
  highShade: string;
  lowShade: string;
  gridColor: string;
}

/**
 * The main timeline canvas where everything is
 */
export class TimelineRenderer extends DynamicCanvas {
  private verticalZoom = 1 / 8;
  private horizontalZoom = 1 / 8;
  private verticalScrollPx = 0;
  private trackCount = 1;
  private subBarDivision = 1;

  constructor(public output: HTMLCanvasElement, public palette: ColorPalette) {
    super(output);
    this.debug(palette);
  }

  /**
   * Set's the canvas' track count
   * @param trackCount
   */
  public setTrackCount(trackCount: number) {
    this.debug(`trackCount: ${trackCount}`);
    // Clamp the value so we don't go below 1
    this.trackCount = Math.max(1, trackCount);
    this.draw();
  }

  /**
   * Adds a track to the canvas
   */
  public addTrack() {
    this.setTrackCount(this.trackCount + 1);
  }

  /**
   * Deletes a track from the canvas
   */
  public deleteTrack() {
    this.setTrackCount(this.trackCount - 1);
  }

  /**
   * Sets the sub bar division to a value between 0.25 and 4
   * @param value the value to set to
   */
  public setSubBarDivision(value: number) {
    this.subBarDivision = minmax(value, 0.25, 4);
    this.debug(this.subBarDivision);
    this.draw();
  }

  /**
   * Raises the sub bar division
   */
  public raiseSubBarDivision() {
    this.setSubBarDivision(this.subBarDivision * 2);
  }

  /**
   * Lowers the sub bar division
   */
  public lowerSubBarDivision() {
    this.setSubBarDivision(this.subBarDivision / 2);
  }

  /**
   * Set's the track vertical zoom to a value
   * @param value the value to set to
   */
  public setVerticalZoom(value: number) {
    this.verticalZoom = value;
    this.draw();
  }

  /**
   * Draws the background of a track
   */
  private drawTrackBackground() {
    const trackHeightPx = this.output.height * this.verticalZoom;

    for (let i = 0; i < this.trackCount; i++) {
      this.ctx.fillStyle = i % 2 == 0 ? this.palette.highShade : this.palette.lowShade; // Alternate track's shades
      this.ctx.fillRect(0, i * trackHeightPx, this.output.width, trackHeightPx);
    }
  }

  /**
   * Draws the sub division bars (beats) within a bar
   * @param barStart the horizontal pixel position of where the current bar starts
   * @param barWidth the total horizontal pixel width of the current bar
   * @param subBarDivision the amount of sub division lines to draw within the bar
   */
  private drawSubBar(barStart: number, barWidth: number, subBarDivision: number = 8) {
    const subBarOffset = barWidth / subBarDivision;

    for (let i = 0; i < subBarDivision; i++) {
      this.ctx.beginPath();
      this.ctx.moveTo(barStart + i * subBarOffset, 0);
      this.ctx.lineTo(barStart + i * subBarOffset, this.output.height);
      this.ctx.stroke();
    }
  }

  /**
   * Draws lines to signify where bars begin, end and their subdivisions
   * @param barsThickness the stroke width of the grid that split the bars
   */
  private drawBars(barsThickness: number = 2) {
    const subBarThickness = barsThickness / 2;

    const gridLineHeightPx = this.output.height * this.verticalZoom;
    const gridLineWidthPx = this.output.width * this.horizontalZoom;
    this.ctx.strokeStyle = this.palette.gridColor;

    for (let i = 0; i < 8; i++) {
      this.ctx.lineWidth = this.px(barsThickness);
      this.ctx.fillStyle = i % 2 == 0 ? this.palette.highShade : this.palette.lowShade;

      // Vertical lines
      this.ctx.beginPath();
      this.ctx.moveTo(i * gridLineWidthPx, 0);
      this.ctx.lineTo(i * gridLineWidthPx, this.output.height);
      this.ctx.stroke();

      // Draw sub vertical lines
      this.ctx.lineWidth = this.px(subBarThickness);
      this.drawSubBar(i * gridLineWidthPx, gridLineWidthPx, 8 * this.subBarDivision);
      this.ctx.lineWidth = this.px(barsThickness);

      // Horizontal lines
      this.ctx.beginPath();
      this.ctx.moveTo(0, i * gridLineHeightPx);
      this.ctx.lineTo(this.output.width, i * gridLineHeightPx);
      this.ctx.stroke();
    }
  }

  public draw() {
    this.ctx.clearRect(0, 0, this.output.width, this.output.height);
    this.drawTrackBackground();
    this.drawBars();
  }
}
