//! Exercise the actual pure parser without linking live MMIO/fence code on host.
#[allow(dead_code)]
#[path = "../src/protocol/registers.rs"]
mod registers;
#[path = "../src/protocol/caps.rs"]
mod caps;
use caps::Caps;

fn snapshot() -> [u8; 256] {
    let mut bytes = [0; 256];
    bytes[0x34] = 0x40;
    for (at, next, len, kind, offset, span) in [
        (0x40, 0x50, 16, 1, 0u32, 56u32),
        (0x50, 0x64, 20, 2, 64, 2),
        (0x64, 0, 16, 4, 128, 8),
    ] {
        bytes[at..at+5].copy_from_slice(&[9, next, len, kind, 2]);
        bytes[at+8..at+12].copy_from_slice(&offset.to_le_bytes());
        bytes[at+12..at+16].copy_from_slice(&span.to_le_bytes());
    }
    bytes[0x60..0x64].copy_from_slice(&4u32.to_le_bytes());
    bytes
}

#[test]
fn exact_bar_bounds_and_alignment_are_required() {
    assert!(Caps::from_parts(0, 64, 4, 128, 136).is_some());
    assert!(Caps::from_parts(0, 64, 4, 128, 135).is_none());
    assert!(Caps::from_parts(0, 65, 4, 128, 136).is_none());
    assert!(Caps::from_parts(0, 0, 4, 128, 136).is_none());
}

#[test]
fn configuration_rejects_loops_short_regions_and_mixed_bar_owners() {
    let bytes = snapshot();
    let (bar, caps) = Caps::read(&bytes).unwrap();
    assert_eq!(bar, 2);
    assert_eq!(caps.parts(), (0, 64, 4, 128));
    let mut cycle = bytes;
    cycle[0x65] = 0x40;
    assert_eq!(Caps::read(&cycle), Err("virtio: caps loop"));
    let mut short = bytes;
    short[0x52] = 19;
    assert_eq!(Caps::read(&short), Err("virtio: caps length"));
    let mut owner = bytes;
    owner[0x54] = 1;
    assert_eq!(Caps::read(&owner), Err("virtio: caps bar"));
    let mut overflow = bytes;
    overflow[0x48..0x4c].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(Caps::read(&overflow), Err("virtio: caps region"));
}
