use std::{
  error::Error,
  fs::{self, OpenOptions},
  io::Write,
  panic::PanicInfo,
  path::PathBuf,
};

use backtrace::Backtrace;

use serde::Serialize;
use serde_json::json;

fn timestamp() -> i64 {
  return chrono::offset::Local::now().timestamp_millis();
}

#[derive(Serialize)]
struct SystemInfo {}

pub struct PanicHandler {
  pub error_filepath: Option<PathBuf>,
}

impl PanicHandler {
  pub fn new() -> Self {
    Self {
      error_filepath: None,
    }
  }

  pub fn handle_panic(&self, panic_info: &PanicInfo) -> Result<(), Box<dyn Error>> {
    let bt = Backtrace::new();

    let output = json!({
      "panic_message": panic_info.to_string(),
      "backtrace": format!("{:?}", &bt),
    });

    if let Some(error_filepath) = &self.error_filepath {
      let fatal_filename = error_filepath.join(format!("{}-fatal.json", timestamp()));
      fs::create_dir_all(error_filepath)?;
      let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .create_new(true)
        .open(fatal_filename)?;
      file.write_all(serde_json::to_string_pretty(&output)?.as_bytes())?;
    }
    std::process::abort();
  }
}
