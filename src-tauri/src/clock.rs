// inspired by https://github.com/mmckegg/rust-loop-drop/blob/master/src/midi_time.rs
// http://www.deluge.co/?q=midi-tempo-bpm

use num::integer::Integer;
use num::rational::Ratio;
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant};

pub type Tick = Ratio<i64>;
pub type Tempo = Ratio<i64>;
pub type NudgeTempo = Ratio<i64>;

static SECONDS_PER_MINUTE: i64 = 60;
static NANOS_PER_SECOND: i64 = 1_000_000_000;

static DEFAULT_TICKS_PER_BEAT: i64 = 32;
static DEFAULT_BEATS_PER_BAR: i64 = 4;
static DEFAULT_BARS_PER_LOOP: i64 = 4;
static DEFAULT_BEATS_PER_MINUTE: i64 = 140;

#[derive(Clone, Copy, Debug)]
pub struct Signature {
  pub ticks_per_beat: Tick,
  pub beats_per_bar: Tick,
  pub bars_per_loop: Tick,
}

impl Signature {
  #[allow(dead_code)]
  pub fn default() -> Self {
    Self {
      ticks_per_beat: Ratio::from_integer(DEFAULT_TICKS_PER_BEAT),
      beats_per_bar: Ratio::from_integer(DEFAULT_BEATS_PER_BAR),
      bars_per_loop: Ratio::from_integer(DEFAULT_BARS_PER_LOOP),
    }
  }

  #[allow(dead_code)]
  pub fn ticks_per_beat(&self) -> Tick {
    self.ticks_per_beat
  }

  #[allow(dead_code)]
  pub fn ticks_per_bar(&self) -> Tick {
    self.ticks_per_beat() * self.beats_per_bar
  }

  #[allow(dead_code)]
  pub fn ticks_per_loop(&self) -> Tick {
    self.ticks_per_bar() * self.bars_per_loop
  }
  #[allow(dead_code)]
  pub fn ticks_to_beats(&self, ticks: Tick) -> Tick {
    ticks / self.ticks_per_beat
  }

  #[allow(dead_code)]
  pub fn ticks_to_bars(&self, ticks: Tick) -> Tick {
    self.ticks_to_beats(ticks) / self.beats_per_bar
  }

  #[allow(dead_code)]
  pub fn nanos_per_tick(&self, beats_per_minute: Tick) -> Tick {
    let minutes_per_beat = Ratio::from_integer(1) / beats_per_minute;
    let seconds_per_beat = minutes_per_beat * Ratio::from_integer(SECONDS_PER_MINUTE);
    let nanos_per_beat = seconds_per_beat * Ratio::from_integer(NANOS_PER_SECOND);
    let nanos_per_tick = nanos_per_beat / self.ticks_per_beat;
    nanos_per_tick
  }

  #[allow(dead_code)]
  pub fn nanos_per_beat(&self, beats_per_minute: Tick) -> Tick {
    self.nanos_per_tick(beats_per_minute) * self.ticks_per_beat
  }

  #[allow(dead_code)]
  pub fn nanos_per_bar(&self, beats_per_minute: Tick) -> Tick {
    self.nanos_per_beat(beats_per_minute) * self.beats_per_bar
  }

  #[allow(dead_code)]
  pub fn nanos_per_loop(&self, beats_per_minute: Tick) -> Tick {
    self.nanos_per_bar(beats_per_minute) * self.bars_per_loop
  }

  #[allow(dead_code)]
  pub fn beats_per_minute(&self, nanos_per_tick: Tick) -> Tempo {
    let nanos_per_beat = nanos_per_tick * self.ticks_per_beat;
    let beats_per_nano = Ratio::from_integer(1) / nanos_per_beat;
    let beats_per_second = beats_per_nano * Ratio::from_integer(NANOS_PER_SECOND);
    let beats_per_minute = beats_per_second * Ratio::from_integer(SECONDS_PER_MINUTE);
    beats_per_minute
  }
}

#[derive(Clone, Copy, Debug)]
pub struct Time {
  ticks: Tick,
  signature: Signature,
}

impl Time {
  #[allow(dead_code)]
  pub fn new(signature: Signature) -> Self {
    Self {
      ticks: Ratio::from_integer(0),
      signature,
    }
  }

  #[allow(dead_code)]
  pub fn ticks(&self) -> Tick {
    self.ticks
  }

  #[allow(dead_code)]
  pub fn beats(&self) -> Tick {
    self.signature.ticks_to_beats(self.ticks)
  }

  #[allow(dead_code)]
  pub fn bars(&self) -> Tick {
    self.signature.ticks_to_bars(self.ticks)
  }

  #[allow(dead_code)]
  pub fn ticks_since_beat(&self) -> Tick {
    self.ticks() % self.signature.ticks_per_beat
  }

  #[allow(dead_code)]
  pub fn beats_since_bar(&self) -> Tick {
    self.beats() % self.signature.beats_per_bar
  }

  #[allow(dead_code)]
  pub fn bars_since_loop(&self) -> Tick {
    self.bars() % self.signature.bars_per_loop
  }

  #[allow(dead_code)]
  pub fn ticks_before_beat(&self) -> Tick {
    self.ticks() - self.ticks_since_beat()
  }

  #[allow(dead_code)]
  pub fn is_first_tick(&self) -> bool {
    self.ticks_since_beat().floor() == Ratio::from_integer(0)
  }

  #[allow(dead_code)]
  pub fn is_first_beat(&self) -> bool {
    self.beats_since_bar().floor() == Ratio::from_integer(0)
  }

  #[allow(dead_code)]
  pub fn is_first_bar(&self) -> bool {
    self.bars_since_loop().floor() == Ratio::from_integer(0)
  }

  #[allow(dead_code)]
  pub fn next(&self) -> Self {
    Self {
      ticks: self.ticks + 1,
      signature: self.signature,
    }
  }

  #[allow(dead_code)]
  pub fn quantize_beat(&self) -> Self {
    // find how far off the beat we are
    let ticks_per_beat = self.signature.ticks_per_beat();
    let ticks_per_half_beat = ticks_per_beat / 2;

    Self {
      // if the beat happened recently
      ticks: if self.ticks_since_beat() < ticks_per_half_beat {
        // nudge back to the beat
        self.ticks_before_beat()
      } else {
        // nudge to the next beat
        self.ticks_before_beat() + ticks_per_beat
      },
      signature: self.signature,
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct Timer {
  instant: Instant,
  signature: Signature,
}

impl Timer {
  #[allow(dead_code)]
  pub fn new(signature: Signature) -> Self {
    Self {
      instant: Instant::now(),
      signature,
    }
  }

  #[allow(dead_code)]
  pub fn nanos(&self) -> Tick {
    Ratio::from_integer(duration_to_nanos(self.instant.elapsed()))
  }

  #[allow(dead_code)]
  pub fn nanos_since_tick(&self, beats_per_minute: Tick) -> Tick {
    self.nanos() % self.signature.nanos_per_tick(beats_per_minute)
  }

  #[allow(dead_code)]
  pub fn nanos_since_beat(&self, beats_per_minute: Tick) -> Tick {
    self.nanos() % self.signature.nanos_per_beat(beats_per_minute)
  }

  #[allow(dead_code)]
  pub fn nanos_since_bar(&self, beats_per_minute: Tick) -> Tick {
    self.nanos() % self.signature.nanos_per_bar(beats_per_minute)
  }

  #[allow(dead_code)]
  pub fn nanos_since_loop(&self, beats_per_minute: Tick) -> Tick {
    self.nanos() % self.signature.nanos_per_loop(beats_per_minute)
  }

  #[allow(dead_code)]
  pub fn nanos_until_tick(&self, beats_per_minute: Tick) -> Tick {
    let nanos_since_tick = self.nanos_since_tick(beats_per_minute);
    let nanos_per_tick = self.signature.nanos_per_tick(beats_per_minute);
    nanos_per_tick - nanos_since_tick
  }

  #[allow(dead_code)]
  pub fn next(&self, beats_per_minute: Tick) -> Tick {
    let nanos_until_tick = self.nanos_until_tick(beats_per_minute);

    let nanos = nanos_until_tick.numer() / nanos_until_tick.denom();

    sleep(Duration::new(0, nanos as u32));

    nanos_until_tick
  }
}

#[derive(Debug)]
pub struct Clock {
  time: Time,
  timer: Timer,
  signature: Signature,
  tempo: Tempo,
  tap: Option<Instant>,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Message {
  Time(Time),
  Tempo(Tempo),
  NudgeTempo(NudgeTempo),
  Reset,
  Signature(Signature),
  Tap,
}

impl Clock {
  #[allow(dead_code)]
  pub fn new() -> Self {
    let signature = Signature::default();
    let time = Time::new(signature);
    let timer = Timer::new(signature);
    let tempo = Ratio::from_integer(DEFAULT_BEATS_PER_MINUTE);

    Self {
      time,
      timer,
      signature,
      tempo,
      tap: None,
    }
  }

  #[allow(dead_code)]
  pub fn start(metronome_tx: Sender<Message>) -> Sender<Message> {
    let mut clock = Self::new();

    let (tx, rx) = channel();

    metronome_tx
      .send(Message::Signature(Signature::default()))
      .unwrap();
    metronome_tx.send(Message::Tempo(clock.tempo)).unwrap();

    spawn(move || {
      loop {
        // wait a tick
        #[allow(unused_variables)]
        let diff = clock.tick();

        // handle any incoming messages
        let mut is_empty = false;
        while !is_empty {
          let message_result = rx.try_recv();
          match message_result {
            Ok(Message::Reset) => {
              clock.reset();
            }
            Ok(Message::Signature(signature)) => {
              clock.set_signature(signature);
            }
            Ok(Message::Tap) => {
              if let Some(new_tempo) = clock.tap() {
                metronome_tx.send(Message::Tempo(new_tempo)).unwrap();
              }
            }
            Ok(Message::NudgeTempo(nudge)) => {
              let old_tempo = clock.tempo;
              let new_tempo = old_tempo + nudge;
              metronome_tx.send(Message::Tempo(new_tempo)).unwrap();
            }
            Ok(Message::Tempo(tempo)) => {
              clock.tempo = tempo;
            }
            Err(TryRecvError::Empty) => {
              is_empty = true;
            }
            Err(TryRecvError::Disconnected) => {
              panic!("{:?}", TryRecvError::Disconnected);
            }
            _ => {}
          }
        }

        // send clock time
        metronome_tx.send(Message::Time(clock.time())).unwrap();
      }
    });

    tx
  }

  #[allow(dead_code)]
  pub fn reset(&mut self) {
    self.time = Time::new(self.signature);
    self.timer = Timer::new(self.signature);
  }

  #[allow(dead_code)]
  pub fn set_signature(&mut self, signature: Signature) {
    self.signature = signature;
    self.time = Time::new(self.signature);
    self.timer = Timer::new(self.signature);
  }

  #[allow(dead_code)]
  pub fn time(&self) -> Time {
    self.time
  }

  #[allow(dead_code)]
  pub fn tick(&mut self) -> Tick {
    let nanos_until_tick = self.timer.next(self.tempo);
    self.time = self.time.next();
    nanos_until_tick
  }

  #[allow(dead_code)]
  pub fn tap(&mut self) -> Option<Tempo> {
    // on every tap, quantize beat
    self.time = self.time.quantize_beat();

    let mut next_tempo = None;

    // if second tap on beat, adjust tempo
    if let Some(tap) = self.tap {
      let tap_nanos = Ratio::from_integer(duration_to_nanos(tap.elapsed()));
      if tap_nanos < self.signature.nanos_per_beat(self.tempo) * 2 {
        let tap_beats_per_nanos = Ratio::from_integer(1) / tap_nanos;
        let tap_beats_per_seconds = tap_beats_per_nanos * Ratio::from_integer(NANOS_PER_SECOND);
        let beats_per_minute = tap_beats_per_seconds * Ratio::from_integer(SECONDS_PER_MINUTE);
        next_tempo = Some(round_to_nearest(beats_per_minute, 100));
      }
    }

    self.tap = Some(Instant::now());

    next_tempo
  }
}

fn duration_to_nanos(duration: Duration) -> i64 {
  duration.as_secs() as i64 * 1_000_000_000 + duration.subsec_nanos() as i64
}

fn round_to_nearest<T: Clone + Copy + Integer>(value: Ratio<T>, quantum: T) -> Ratio<T> {
  let quantum_rat = Ratio::from_integer(quantum);
  (value * quantum_rat).round() / quantum_rat
}
