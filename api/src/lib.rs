#![no_std]
//! Binding wire contract and activation coordinates owned by the VirtIO Driver.

mod call;
mod coords;
mod observe;
mod said;
mod supply;
mod wire;

pub use call::Call;
pub use coords::Coords;
pub use observe::Observe;
pub use said::Said;
pub use supply::Supply;
pub use wire::Wire;

pub const ENTRY: env::Mark = env::Mark::of("virtio-blk/binding");
pub const BACK: env::Mark = env::Mark::of("virtio-blk/binding-back");
pub const OBSERVE: u8 = 1;
pub const SUPPLY: u8 = 2;
