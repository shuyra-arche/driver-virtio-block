//! Private live virtio-pci register protocol; mappings are held by the executor.

mod caps;
pub mod device;
mod registers;

pub use caps::Caps;
pub use device::{Virtio, notify_offset, queue_enable, reset, ring, status};
pub use registers::NO_VECTOR;
