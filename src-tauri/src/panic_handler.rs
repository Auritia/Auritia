use std::{
  fs::{self, File, OpenOptions},
  io::Write,
  panic::PanicInfo,
  path::PathBuf,
  sync::Arc,
};

use backtrace::Backtrace;

use anyhow::Result;
use native_dialog::{MessageDialog, MessageType};
use once_cell::sync::OnceCell;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

use crate::util::timestamp;

#[derive(Error, Debug)]
pub enum PanicHandlerError {
  #[error("Error file path not set")]
  FilePathNotSet,
  #[error("Error file has no parent")]
  FileHasNoParent,
}

#[derive(Serialize)]
struct SystemInfo {}

pub struct PanicHandler {
  error_filepath: Arc<OnceCell<PathBuf>>,
}

impl PanicHandler {
  pub fn new(path_cell: Arc<OnceCell<PathBuf>>) -> Self {
    Self {
      error_filepath: path_cell,
    }
  }

  pub fn handle_panic(&self, panic_info: &PanicInfo) -> Result<()> {
    let bt = Backtrace::new();

    let output = json!({
      "panic_message": panic_info.to_string(),
      "backtrace": format!("{:?}", &bt),
    });
    let error_filepath = self.new_error_path()?;
    self
      .new_error_file(&error_filepath)?
      .write_all(serde_json::to_string_pretty(&output)?.as_bytes())?;

    println!(
      "PANIC: {:?}. Full log can be found at {:?}",
      panic_info, &error_filepath,
    );

    MessageDialog::new()
      .set_type(MessageType::Error)
      .set_title("Auritia Panic")
      .set_text(&format!(
        "{:#?}\nView additional debug information at {:?}",
        panic_info.to_string(),
        &error_filepath
      ))
      .show_alert()?;

    std::process::abort();
  }

  fn new_error_path(&self) -> Result<PathBuf> {
    let error_dir = self
      .error_filepath
      .get()
      .ok_or(PanicHandlerError::FilePathNotSet)?;

    Ok(error_dir.join(format!("{}-fatal.json", timestamp())))
  }

  fn new_error_file(&self, path: &PathBuf) -> Result<File> {
    fs::create_dir_all(path.parent().ok_or(PanicHandlerError::FileHasNoParent)?)?;

    let file = OpenOptions::new()
      .write(true)
      .read(true)
      .create(true)
      .create_new(true)
      .open(path)?;

    Ok(file)
  }
}
