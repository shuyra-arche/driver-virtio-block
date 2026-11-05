#![no_std]
//! Live VirtIO device protocol, binding coordination and BAR recovery.

extern crate alloc;

mod coord;
mod domain_id;
mod exchange;
mod lookup;
mod operation;
mod order;
mod peers;
mod polled;
mod profile;
mod protocol;
mod recovery;
mod serve;
mod stage;

pub use serve::serve;
