#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat};
use std::str::FromStr;
use std::sync::RwLock;
use tauri::Manager;

#[macro_use]
extern crate lazy_static;

fn load_metronome() -> Vec<Vec<f32>> {
  return vec![
    load_sample("./sounds/metronome_low.wav"),
    load_sample("./sounds/metronome_high.wav"),
  ];
}

fn load_sample(path: &str) -> Vec<f32> {
  return hound::WavReader::open(path)
    .unwrap()
    .samples::<f32>()
    .map(|s| s.unwrap())
    .collect();
}

lazy_static! {
  static ref IS_METRONOME_ENABLED: RwLock<bool> = RwLock::new(false);
  static ref IS_PLAYING: RwLock<bool> = RwLock::new(false);
  static ref BPM: RwLock<f32> = RwLock::new(120.00);
  static ref SAMPLE_RATE: RwLock<u32> = RwLock::new(44100);
  static ref CHANNEL_COUNT: RwLock<u16> = RwLock::new(2);
  static ref METRONOME_SOUND: Vec<Vec<f32>> = load_metronome();
}

mod interface;

// the payload type must implement `Serialize`.
// for global events, it also must implement `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  value: String,
}

fn write_data<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
  // For each sample of the current sample rate iteration write white noise
  for sample in data.iter_mut() {
    let mut current_sample = 0.0;

    if *IS_PLAYING.read().unwrap() == true {
      if *IS_METRONOME_ENABLED.read().unwrap() == true {
        let white_noise_sample = (rand::random::<f32>() - 0.5) / 2.0;
        current_sample = white_noise_sample;
      }
    }
    *sample = Sample::from(&current_sample);
  }
}

fn main() {
  // The default host for the current compilation target platform
  let host = cpal::default_host();

  // Get the default output audio device on the system
  let device = host
    .default_output_device()
    .expect("no output device available");

  println!("[DEBUG] Got device: {}", device.name().unwrap());

  // Get supported stream formats by the device
  let mut supported_configs_range = device
    .supported_output_configs()
    .expect("error while querying configs");

  // Get the supported config
  let supported_config = supported_configs_range
    .next()
    .expect("no supported config?!")
    .with_max_sample_rate();

  // An error handler to handle write errors on stream
  let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

  // Get configs
  let sample_format = supported_config.sample_format();
  let config: cpal::StreamConfig = supported_config.into();

  // Update the sample rate with the device's default :trol:
  *SAMPLE_RATE.write().unwrap() = config.sample_rate.0;
  println!("[DEBUG] Device Sample Rrate: {}", config.sample_rate.0);
  // Update the channels with the device's default :trol:
  *CHANNEL_COUNT.write().unwrap() = config.channels;
  println!("[DEBUG] Device channels: {}", config.channels);

  println!("[DEBUG] Device Buffer Size: {:?}", config.buffer_size);

  // Create a stream for the corresponding format
  let stream = match sample_format {
    SampleFormat::F32 => device.build_output_stream(&config, write_data::<f32>, err_fn),
    SampleFormat::I16 => device.build_output_stream(&config, write_data::<i16>, err_fn),
    SampleFormat::U16 => device.build_output_stream(&config, write_data::<u16>, err_fn),
  }
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
        println!("[EVENTS] got set_metronome with payload {:?}", value);
      });

      app.listen_global("play", |_| {
        *IS_PLAYING.write().unwrap() = true;
      });

      app.listen_global("stop", |_| {
        *IS_PLAYING.write().unwrap() = false;
      });

      // unlisten to the event using the `id` returned on the `listen_global` function
      Ok(())
    })
    // Run the app
    .run(tauri::generate_context!())
    // Catch errors
    .expect("error while running tauri application");
}
