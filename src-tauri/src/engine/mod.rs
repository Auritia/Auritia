use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Sample, SampleFormat};

#[tauri::command]
pub fn beep() {
  // The default host for the current compilation target platform
  let host = cpal::default_host();

  // Get the default output audio device on the system
  let device = host
    .default_output_device()
    .expect("no output device available");

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
  let config = supported_config.into();

  println!("playing");

  // Create a stream for the corresponding format
  match sample_format {
    SampleFormat::F32 => device.build_output_stream(&config, write_silence::<f32>, err_fn),
    SampleFormat::I16 => device.build_output_stream(&config, write_silence::<i16>, err_fn),
    SampleFormat::U16 => device.build_output_stream(&config, write_silence::<u16>, err_fn),
  }
  .unwrap();
}

fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
  // For each sample of the current sample rate iteration write silence
  for sample in data.iter_mut() {
    *sample = Sample::from(&0.0);
  }
}
