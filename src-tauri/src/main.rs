#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

extern crate ringbuf;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::RingBuffer;
use std::str::FromStr;
use std::sync::mpsc::channel;
use std::sync::RwLock;
use std::thread::spawn;
use std::time::Duration;
use std::time::SystemTime;
use tauri::Manager;

mod clock;

#[macro_use]
extern crate lazy_static;

fn load_metronome() -> Vec<Wav> {
  return vec![
    load_sample("./sounds/metronome_high.wav"),
    load_sample("./sounds/metronome_low.wav"),
  ];
}

fn load_sample(path: &str) -> Wav {
  let samples = hound::WavReader::open(path)
    .unwrap()
    .samples::<f32>()
    .map(|s| s.unwrap())
    .collect();

  return Wav {
    sample_rate: 44100,
    channel_count: 2,
    current_sample: 0,
    samples: samples,
  };
}

lazy_static! {
  static ref START_TIME: SystemTime = SystemTime::now();
  static ref IS_METRONOME_ENABLED: RwLock<bool> = RwLock::new(false);
  static ref IS_PLAYING: RwLock<bool> = RwLock::new(false);
  static ref BPM: RwLock<f32> = RwLock::new(150.00);
  static ref SAMPLE_RATE: RwLock<u32> = RwLock::new(44100);
  static ref CHANNEL_COUNT: RwLock<u16> = RwLock::new(2);
  static ref METRONOME_SOUND: RwLock<Vec<Wav>> = RwLock::new(load_metronome());
}

// the payload type must implement `Serialize`.
// for global events, it also must implement `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  value: String,
}

#[derive(Clone)]
struct Wav {
  sample_rate: u32,
  channel_count: u16,
  samples: Vec<f32>,
  current_sample: usize,
}

pub fn calc_beat_delta(bpm: u16, lower: u8) -> Duration {
  let quarter_note_sec: f64 = 60f64 / bpm as f64;
  let factor: f64 = 4f64 / lower as f64;

  Duration::from_secs_f64(quarter_note_sec * factor)
}

#[allow(dead_code)]
fn mix_waves(waves: Vec<f32>) -> f32 {
  let mut value: f32 = 0.0;
  for i in 0..waves.len() {
    value += waves[i];
  }
  return value;
}

fn main() {
  // The default host for the current compilation target platform
  let host = cpal::default_host();

  // Get the default output audio device on the system
  let device = host
    .default_output_device()
    .expect("no output device available");

  println!("[DEBUG] Got device: {}", device.name().unwrap());

  // An error handler to handle write errors on stream
  let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

  // The buffer to share samples
  let ring = RingBuffer::<f32>::new(1024);
  let (mut producer, mut consumer) = ring.split();
  let mut metronome_sound_high = METRONOME_SOUND.read().unwrap()[0].clone();
  let mut metronome_sound_low = METRONOME_SOUND.read().unwrap()[1].clone();
  spawn(move || {
    let (tx, rx) = channel();
    let clock_tx = clock::Clock::start(tx.clone());
    for control_message in rx {
      match control_message {
        // // sent by interface
        // Message::Reset => {
        //   clock_tx.send(clock::Message::Reset).unwrap();
        // }
        // // sent by interface
        // Message::NudgeTempo(nudge) => {
        //   clock_tx.send(clock::Message::NudgeTempo(nudge)).unwrap();
        // }
        // // sent by interface
        // Message::Tap => {
        //   clock_tx.send(clock::Message::Tap).unwrap();
        // }
        // // sent by clock
        // Message::Signature(signature) => {
        //   clock_tx.send(clock::Message::Signature(signature)).unwrap();

        // // sent by clock
        // Message::Tempo(tempo) => {
        //   clock_tx.send(clock::Message::Tempo(tempo)).unwrap();
        // }
        // Send an event every tick
        clock::Message::Time(time) => {
          // If we are at the start of the beat play a metronome sound
          if time.ticks_since_beat().to_integer() == 0 {
            println!("BEAT");

            if *IS_PLAYING.read().unwrap() == true {
              if *IS_METRONOME_ENABLED.read().unwrap() == true {
                // High
                if time.beats_since_bar().to_integer() == 0 {
                  write(&mut producer, &metronome_sound_high.samples);
                }
                // Low
                else {
                  write(&mut producer, &metronome_sound_low.samples);
                }
              }
            }
          }

          print_time(time);
          // terminal_tx.send(interface::Message::Time(time)).unwrap();
        }
        _ => {}
      }
    }
  });

  // Get configs
  let config: cpal::StreamConfig = cpal::StreamConfig {
    channels: 2,
    sample_rate: cpal::SampleRate(44100),
    buffer_size: cpal::BufferSize::Default,
  };

  let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
    for sample in data {
      *sample = match consumer.pop() {
        Some(s) => s,
        None => 0.0,
      };
    }
  };

  // Update the sample rate with the device's default :trol:
  *SAMPLE_RATE.write().unwrap() = config.sample_rate.0;
  println!("[DEBUG] Device Sample Rrate: {}", config.sample_rate.0);
  // Update the channels with the device's default :trol:
  *CHANNEL_COUNT.write().unwrap() = config.channels;
  println!("[DEBUG] Device channels: {}", config.channels);

  println!("[DEBUG] Device Buffer Size: {:?}", config.buffer_size);

  // Create a stream for the corresponding format
  let stream = device
    .build_output_stream(&config, output_data_fn, err_fn)
    .unwrap();

  stream.play().unwrap();

  tauri::Builder::default()
    // Register Rust function to Vue
    // .invoke_handler(tauri::generate_handler![engine::create])
    .setup(|app| {
      // listen to the `event-name` (emitted on any window)
      app.listen_global("set_metronome", |event| {
        let value: bool = FromStr::from_str(event.payload().unwrap()).unwrap();
        *IS_METRONOME_ENABLED.write().unwrap() = value;
        println!("[EVENTS] got 'set_metronome' with payload {:?}", value);
      });

      app.listen_global("set_bpm", |event| {
        let value: f32 = FromStr::from_str(event.payload().unwrap()).unwrap();
        *BPM.write().unwrap() = value;
        println!("[EVENTS] got 'set_bpm' with payload {:?}", value);
      });

      app.listen_global("play", |_| {
        *IS_PLAYING.write().unwrap() = true;
        println!("[EVENTS] got 'play'");
      });

      app.listen_global("stop", |_| {
        *IS_PLAYING.write().unwrap() = false;
        println!("[EVENTS] got 'stop'");
      });

      // unlisten to the event using the `id` returned on the `listen_global` function
      Ok(())
    })
    // Run the app
    .run(tauri::generate_context!())
    // Catch errors
    .expect("error while running tauri application");
}

pub fn print_time(time: clock::Time) {
  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
  let ticks_since_beat = time.ticks_since_beat();
  println!("ticks since beat: {}", &ticks_since_beat);
  if ticks_since_beat.to_integer() == 0 {
    println!("BEAT");
  } else {
    for i in 0..ticks_since_beat.to_integer() {
      print!("-");
    }
  }
}

fn write(producer: &mut ringbuf::Producer<f32>, samples: &Vec<f32>) {
  for i in 0..samples.len() {
    if i < 1024 {
      producer.push(samples[i]).unwrap();
    }
  }
}
