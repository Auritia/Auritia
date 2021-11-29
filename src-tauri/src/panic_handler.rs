use std::{
  error::Error,
  fs::{self, File, OpenOptions},
  io::Write,
  panic::PanicInfo,
  path::PathBuf,
};

fn timestamp() -> i64 {
  return chrono::offset::Local::now().timestamp_millis();
}

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
    if let Some(error_filepath) = &self.error_filepath {
      let fatal_filename = error_filepath.join(format!("{}-fatal.acf", timestamp()));
      println!("{:?}", error_filepath);
      println!("{:?}", fatal_filename);

      fs::create_dir_all(error_filepath)?;
      let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .create_new(true)
        .open(error_filepath)?;
      file.write_all(panic_info.to_string().as_bytes())?;
    }
    eprintln!("{}", panic_info);
    std::process::abort();
  }
}
