use num::rational::Ratio;
use std::sync::mpsc::{channel, Sender};
use std::thread::spawn;

use crate::clock;
use crate::interface;
use crate::metronome;

// https://unicode.org/charts/PDF/U0000.pdf
static CHAR_SPACE: u32 = 0x0020;
#[allow(dead_code)]
static CHAR_RETURN: u32 = 0x000D;
static CHAR_NEWLINE: u32 = 0x000A;

#[derive(Debug)]
pub struct Terminal {}

impl Terminal {
  pub fn start(metronome_tx: Sender<metronome::Message>) -> Sender<Message> {
    let (tx, rx) = channel();

    let mut signature = clock::Signature::default();
    let mut tempo = Ratio::from_integer(0);

    spawn(move || {
      for interface_message in rx {
        match interface_message {
          Message::Time(time) => {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            if time.ticks_since_beat().to_integer() == 0 {
              // println!("BEAT");
            }
            print_time(time);
            // print_signature(signature);
            // print_tempo(tempo);
          }
          Message::Signature(next_signature) => {
            signature = next_signature;
          }
          Message::Tempo(next_tempo) => {
            tempo = next_tempo;
          }
        }
      }
    });

    tx
  }
}

pub fn print_time(time: clock::Time) {
  let ticks_since_beat = time.ticks_since_beat();
  println!("ticks since beat: {}", &ticks_since_beat);
  if ticks_since_beat.to_integer() == 0 {
    println!("BEAT");
  } else {
    for i in 0..ticks_since_beat.to_integer() {
      println!("-");
    }
  }
}

pub fn print_signature(signature: clock::Signature) {
  println!("ticks per beat: {}", &signature.ticks_per_beat);
  println!("beats per bar: {}", &signature.beats_per_bar);
  println!("bars per loop: {}", &signature.bars_per_loop);
}

pub fn print_tempo(tempo: clock::Tempo) {
  println!("beats per minute: {:?}", tempo.to_integer());
}

#[derive(Clone, Copy, Debug)]
pub enum Message {
  Time(clock::Time),
  Signature(clock::Signature),
  Tempo(clock::Tempo),
}
