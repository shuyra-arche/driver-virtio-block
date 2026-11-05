#![no_std]
#![no_main]

//! VirtIO block Driver image: the body lives in the server library's `serve`.

#[mold::entry]
fn main() -> execution::boot::Report<'static> {
    match virtio_blk_server::serve() {
        Ok(()) => execution::boot::Report::note(env::EXIT_OK, "virtio-blk: done"),
        Err(why) => execution::boot::Report::note(env::EXIT_FAULT, why),
    }
}
