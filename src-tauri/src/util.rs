use parking_lot::Mutex;
use std::sync::Arc;

pub fn arcmutex<T>(item: T) -> Arc<Mutex<T>> {
  return Arc::new(Mutex::new(item));
}
