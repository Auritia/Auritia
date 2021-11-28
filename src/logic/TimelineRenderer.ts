import { tsMethodSignature } from "@babel/types";
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

  constructor(public output: HTMLCanvasElement, public palette: ColorPalette) {
    super(output);
    this.debug(palette);
  }

  private drawTrackHighlights() {
    const trackHeightPx = this.output.height * this.verticalZoom;

    for (let i = 0; i < 8; i++) {
      // whatever default value for now
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
      this.drawSubBar(i * gridLineWidthPx, gridLineWidthPx, 8);
      this.ctx.lineWidth = this.px(barsThickness);

      // Horizontal lines
      this.ctx.beginPath();
      this.ctx.moveTo(0, i * gridLineHeightPx);
      this.ctx.lineTo(this.output.width, i * gridLineHeightPx);
      this.ctx.stroke();
    }
  }

  public draw() {
    this.drawTrackHighlights();
    this.drawBars(2);
  }
}
