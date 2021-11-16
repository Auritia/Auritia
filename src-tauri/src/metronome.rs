use std::sync::mpsc::{channel, Receiver, Sender};

use crate::clock;
use crate::interface;

#[derive(Clone, Copy, Debug)]
pub enum Message {
  Time(clock::Time),
  Signature(clock::Signature),
  Tempo(clock::Tempo),
  Reset,
  NudgeTempo(clock::NudgeTempo),
  Tap,
  /*
  Stop,
  NudgeClock,
  Configure
  */
}

#[derive(Debug)]
pub struct Metronome {
  pub tx: Sender<Message>,
  pub rx: Receiver<Message>,
}

impl Metronome {
  pub fn new() -> Self {
    let (tx, rx) = channel();

    Self { tx, rx }
  }

  pub fn run(self) {
    let terminal_tx = interface::Terminal::start(self.tx.clone());
    let clock_tx = clock::Clock::start(self.tx.clone());

    for control_message in self.rx {
      match control_message {
        // sent by interface
        Message::Reset => {
          clock_tx.send(clock::Message::Reset).unwrap();
        }
        // sent by interface
        Message::NudgeTempo(nudge) => {
          clock_tx.send(clock::Message::NudgeTempo(nudge)).unwrap();
        }
        // sent by interface
        Message::Tap => {
          clock_tx.send(clock::Message::Tap).unwrap();
        }
        // sent by clock
        Message::Signature(signature) => {
          clock_tx.send(clock::Message::Signature(signature)).unwrap();
          terminal_tx
            .send(interface::Message::Signature(signature))
            .unwrap();
        }
        // sent by clock
        Message::Tempo(tempo) => {
          clock_tx.send(clock::Message::Tempo(tempo)).unwrap();
          terminal_tx.send(interface::Message::Tempo(tempo)).unwrap();
        }
        // Send an event every tick
        Message::Time(time) => {
          terminal_tx.send(interface::Message::Time(time)).unwrap();
        }
      }
    }
  }
}
