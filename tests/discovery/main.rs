#![no_std]
#![no_main]

extern crate alloc;

use env::{PieToken, Reason, Wait};
use system_api::operator::Path;
use system_client::operator::Face;

const WAIT: Wait = Wait::AtMost(1000);

#[mold::entry]
fn main() -> Result<(), Reason> {
    let sire = env::unit::sire();
    let session = ipc::session::Session::open(sire, system_client::operator::BERTH, WAIT)
        .map_err(|_| 1usize)?;
    let tree = Face::of(session);
    let service = tree
        .resolve(system_api::namespace::SERVICE, WAIT)
        .map_err(|_| 2usize)?;
    let state_path = system_api::control::DIR
        .try_join(system_api::control::Grant::State.name())
        .ok_or(3usize)?;
    let token = tree
        .tile_from(service, &state_path, WAIT)
        .and_then(|tile| tile.token(WAIT))
        .map_err(|_| 4usize)?;
    if ipc::session::establish::marked_as(token) != Some(system_api::control::Grant::State.mark()) {
        return Err(5);
    }
    let state = system_client::control::Face::of_owned(token, sire).map_err(|_| 6usize)?;
    let hub = state
        .publisher(
            system_api::namespace::SERVICE,
            hub_api::Grant::Dma.mark(),
            WAIT,
        )
        .map_err(|_| 7usize)?;
    let entry = tree
        .tile_from(service, Path::new("hub/list"), WAIT)
        .and_then(|tile| tile.token(WAIT))
        .map_err(|_| 8usize)?;
    let result = listed(entry, hub);
    env::pie::release(entry, env::ReleaseMode::Revoke).map_err(|_| 14usize)?;
    result
}

fn listed(entry: PieToken, hub: env::TaskId) -> Result<(), Reason> {
    let face = hub_client::Face::of(entry).map_err(|_| 9usize)?;
    if face.host() != hub
        || ipc::session::establish::marked_as(entry) != Some(hub_api::Grant::List.mark())
    {
        return Err(10);
    }
    let deadline = env::chrono::clock().saturating_add(5_000_000_000);
    loop {
        match face.list(alloc::string::String::from("virtio-blk"), 0, WAIT) {
            Ok(window)
                if window.n == 1
                    && window
                        .name(0)
                        .is_some_and(|name| name == "virtio-blk-pci-1") =>
            {
                return Ok(());
            }
            Ok(window) if window.n == 0 => {}
            Err(hub_api::Fail::Unknown) => {}
            _ => return Err(11),
        }
        if env::chrono::clock() >= deadline {
            return Err(12);
        }
        execution::room::park(core::time::Duration::from_millis(1)).map_err(|_| 13usize)?;
    }
}
