import { minmax } from ".";
import { DynamicCanvas } from "./DynamicCanvas";

export interface ColorPalette {
  highShade: string;
  lowShade: string;
  gridColor: string;
}

export class TimelineTrack {
  // prettier-ignore
  constructor(
    public name: string = "Track", 
    public color: string = `#${Math.floor(Math.random()*16777215).toString(16)}`,
    public volume: number = 100.0,
    ) {

  }
}

/**
 * The main timeline canvas where everything is
 */
export class TimelineRenderer extends DynamicCanvas {
  private verticalZoom = 1 / 8;
  private horizontalZoom = 1 / 8;
  private verticalScrollPx = 0;
  private subBarDivision = 1;
  public tracks: TimelineTrack[] = [new TimelineTrack()];

  constructor(public canvas: HTMLCanvasElement, public palette: ColorPalette) {
    super(canvas);
    this.debug(palette);
  }

  /**
   * Adds a track to the canvas
   */
  public addTrack() {
    this.tracks.push(new TimelineTrack());
    this.draw();
  }

  /**
   * Deletes a track from the canvas
   */
  public deleteTrack(idx: number) {
    // delete this.tracks[idx];
    this.tracks = []; // delete all for now
    this.draw();
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
   * Draws the background of a bar
   */
  private drawBarBackground() {
    const barWidth = this.canvas.width * this.horizontalZoom;
    for (let i = 0; i < 1 / this.horizontalZoom; i++) {
      const barStart = i * barWidth;
      this.ctx.fillStyle = i % 2 == 0 ? this.palette.highShade : this.palette.lowShade; // Alternate bar's shades
      this.ctx.fillRect(barStart, 0, barStart + barWidth, this.canvas.height);
    }
  }

  /**
   * Draws the sub division bars (beats) within a bar
   * @param barStart the horizontal pixel position of where the current bar starts
   * @param barWidth the total horizontal pixel width of the current bar
   * @param subBars the amount of sub bars to draw within the bar
   */
  private drawSubBar(barStart: number, barWidth: number, subBars: number = 8 * this.subBarDivision) {
    const subBarOffset = barWidth / subBars;

    for (let i = 0; i < subBars; i++) {
      this.ctx.beginPath();
      this.ctx.moveTo(barStart + i * subBarOffset, 0);
      this.ctx.lineTo(barStart + i * subBarOffset, this.canvas.height);
      this.ctx.stroke();
    }
  }

  /**
   * Proof of concept to see how it would look if it was done in the canvas
   *
   * @todo This should be done with DOM elements for clarity and performance
   *       because we would also have to have the VU meters within this so it
   *       would be kinda dumb to have a canvas within a canvas sort of say
   * @deprecated
   */
  private drawTrackControls(track: TimelineTrack, width: number, y: number, h: number) {
    const boxWidth = this.px(width);

    // Draw track box
    this.ctx.fillStyle = track.color;
    this.ctx.fillRect(this.canvas.width - boxWidth, y, boxWidth, h);
    this.ctx.fillStyle = "#fff";
    this.ctx.font = `${this.px(12)}px Arial`;
    this.ctx.fillText(track.name, this.canvas.width - boxWidth + this.px(8), y + this.px(16));
  }

  public drawTracks() {
    const trackHeight = this.canvas.height * this.verticalZoom;

    // Horizontal lines for each track
    for (let i = 0; i < this.tracks.length; i++) {
      const track = this.tracks[i];

      const trackVerticalPositionOnScreen = i * trackHeight;

      // Draw lines
      this.ctx.beginPath();
      this.ctx.moveTo(0, trackVerticalPositionOnScreen + trackHeight);
      this.ctx.lineTo(this.canvas.width, trackVerticalPositionOnScreen + trackHeight);
      this.ctx.stroke();

      // Track the track's ontrols
      this.drawTrackControls(track, 168, trackVerticalPositionOnScreen, trackHeight);
    }
  }

  /**
   * Draws lines to signify where bars begin, end and their subdivisions
   * @param barsThickness the stroke width of the grid that split the bars
   */
  private drawBars(barsThickness: number = 2) {
    const subBarThickness = barsThickness / 2;

    const gridLineWidthPx = this.canvas.width * this.horizontalZoom;
    this.ctx.strokeStyle = this.palette.gridColor;

    for (let i = 0; i < 1 / this.horizontalZoom; i++) {
      this.ctx.lineWidth = this.px(barsThickness);
      this.ctx.fillStyle = i % 2 == 0 ? this.palette.highShade : this.palette.lowShade;

      // Vertical lines
      this.ctx.beginPath();
      this.ctx.moveTo(i * gridLineWidthPx, 0);
      this.ctx.lineTo(i * gridLineWidthPx, this.canvas.height);
      this.ctx.stroke();

      // Draw sub vertical lines
      this.ctx.lineWidth = this.px(subBarThickness);
      this.drawSubBar(i * gridLineWidthPx, gridLineWidthPx);
      this.ctx.lineWidth = this.px(barsThickness);
    }
  }

  public draw() {
    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
    this.drawBarBackground();
    this.drawBars();
    this.drawTracks();
  }
}
