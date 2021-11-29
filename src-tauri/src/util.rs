use parking_lot::Mutex;
use std::sync::Arc;

/// Creates an Arc Mutex
pub fn arcmutex<T>(item: T) -> Arc<Mutex<T>> {
  return Arc::new(Mutex::new(item));
}
