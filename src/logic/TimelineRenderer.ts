import { onKeyStroke, useKeyModifier } from "@vueuse/core";
import { DynamicCanvas } from "./DynamicCanvas";

export interface ColorPalette {
  highShade: string;
  lowShade: string;
  gridColor: string;
}

export class TimelineRenderer extends DynamicCanvas {
  private verticalZoom = 1 / 8;
  private horizontalZoom = 1 / 8;
  private verticalScrollPx = 0;
  private trackCount = 1;
  private subBarDivision = 1;

  constructor(public output: HTMLCanvasElement, public palette: ColorPalette) {
    super(output);
    this.debug(palette);
    const crtl = useKeyModifier("Control");
    onKeyStroke("1", () => crtl.value && this.lowerSubBarDivision());
    onKeyStroke("2", () => crtl.value && this.raiseSubBarDivision());
  }

  public setTrackCount(trackCount: number) {
    this.debug(`trackCount: ${trackCount}`);
    // Clamp the value so we don't go below 1
    this.trackCount = Math.max(1, trackCount);
    this.draw();
  }

  public addTrack() {
    this.setTrackCount(this.trackCount + 1);
  }

  public deleteTrack() {
    this.setTrackCount(this.trackCount - 1);
  }

  public setSubBarDivision(value: number) {
    this.subBarDivision = Math.min(Math.max(value, 1), 8);
    this.debug(this.subBarDivision);
    this.draw();
  }

  public raiseSubBarDivision() {
    this.setSubBarDivision(this.subBarDivision + 1);
  }

  public lowerSubBarDivision() {
    this.setSubBarDivision(this.subBarDivision - 1);
  }

  public setVerticalZoom(value: number) {
    this.verticalZoom = value;
    this.draw();
  }

  private drawTrackHighlights() {
    const trackHeightPx = this.output.height * this.verticalZoom;

    for (let i = 0; i < this.trackCount; i++) {
      this.ctx.fillStyle = i % 2 == 0 ? this.palette.highShade : this.palette.lowShade; // Alternate track's shades
      this.ctx.fillRect(0, i * trackHeightPx, this.output.width, trackHeightPx);
    }
  }

  private drawSubBar(barStart: number, barWidth: number, gridSize: number = 8) {
    const subBarOffset = barWidth / gridSize;

    for (let i = 0; i < gridSize; i++) {
      this.ctx.beginPath();
      this.ctx.moveTo(barStart + i * subBarOffset, 0);
      this.ctx.lineTo(barStart + i * subBarOffset, this.output.height);
      this.ctx.stroke();
    }
  }

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
    this.drawTrackHighlights();
    this.drawBars(2);
  }
}
