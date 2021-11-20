#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

extern crate ringbuf;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use num::rational::Ratio;
use ringbuf::RingBuffer;
use std::str::FromStr;
use std::sync::mpsc::channel;
use std::sync::RwLock;
use std::thread;
use std::time::SystemTime;
use tauri::Manager;

mod clock;

static RING_BUFFER_SIZE: usize = 2048;

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
  let ring = RingBuffer::<f32>::new(RING_BUFFER_SIZE);
  let (mut producer, mut consumer) = ring.split();
  let metronome_sound_high = METRONOME_SOUND.read().unwrap()[0].clone();
  let metronome_sound_low = METRONOME_SOUND.read().unwrap()[1].clone();

  // Create a channel to receive messages from other threads
  let (tx, rx) = channel();

  let clock_tx = clock::Clock::start(tx.clone());

  thread::Builder::new()
    .name("event_handler".to_string())
    .spawn(move || {
      // Make another sender and pass it to the clock so
      // the clock can send events to the main thread as well

      // Listen to the events in the main thread
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
          // Update the UI whenever the clock sends an event that the tempo changed
          clock::Message::Tempo(tempo) => {
            println!("{:?}", tempo);
          }
          // Send an event every tick
          clock::Message::Time(time) => {
            let current_bar = time.bars().to_integer();
            let current_beat = time.beats_since_bar().to_integer() + 1;

            // If we are at the start of the beat play a metronome sound
            if time.ticks_since_beat().to_integer() == 0 {
              if *IS_PLAYING.read().unwrap() {
                if *IS_METRONOME_ENABLED.read().unwrap() {
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
          }
          _ => {}
        }
      }
    })
    .expect("event_handler did a little trolling");

  // let sample_format = supported_config.sample_format();
  let mut supported_configs_range = device
    .supported_output_configs()
    .expect("error while querying configs");
  let supported_config = supported_configs_range
    .next()
    .expect("no supported config?!")
    .with_max_sample_rate();
  let config: cpal::StreamConfig = supported_config.into();

  // This function runs by CPAl to write data to the audio stream
  // by popping each sample off the ringbuffer array
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

  // Assures the stream starts playing because some low-end devices
  // don't start it by default apparently
  stream.play().unwrap();

  let events = ["set_metronome", "tap_metronome", "set_bpm", "play", "stop"];

  // Creates the webapp
  tauri::Builder::default()
    .setup(move |app| {
      for event_name in events {
        let tx = clock_tx.clone();

        match event_name {
          "set_metronome" => {
            app.listen_global(event_name, move |event| {
              let value: bool = FromStr::from_str(event.payload().unwrap()).unwrap();
              *IS_METRONOME_ENABLED.write().unwrap() = value;
              println!("[EVENTS] got '{}' with payload {:?}", event_name, value);
            });
          }
          "tap_metronome" => {
            app.listen_global(event_name, move |_| {
              tx.send(clock::Message::Tap).unwrap();
              println!("[EVENTS] got '{}'", event_name);
            });
          }
          "set_bpm" => {
            app.listen_global(event_name, move |event| {
              // This crashes when incementing by 0.10
              let value: i64 = FromStr::from_str(event.payload().unwrap()).unwrap();
              tx.send(clock::Message::Tempo(Ratio::from_integer(value)))
                .unwrap();
              println!("[EVENTS] got '{}' with payload {:?}", event_name, value);
            });
          }
          "play" => {
            app.listen_global(event_name, move |_| {
              *IS_PLAYING.write().unwrap() = true;
              println!("[EVENTS] got '{}'", event_name);
            });
          }
          "stop" => {
            app.listen_global(event_name, move |_| {
              *IS_PLAYING.write().unwrap() = false;
              println!("[EVENTS] got '{}'", event_name);
            });
          }
          _ => {}
        }
      }
      Ok(())
    })
    // Register Rust function to Vue
    // .invoke_handler(tauri::generate_handler![engine::create])
    // Run the app
    .run(tauri::generate_context!())
    // Catch errors
    .expect("error while running tauri application");
}

fn write(producer: &mut ringbuf::Producer<f32>, samples: &Vec<f32>) {
  for i in 0..samples.len() {
    // This currently chops of the metronome if the buffer size is too low
    if i < RING_BUFFER_SIZE {
      producer.push(samples[i]).unwrap();
    }
  }
}
