use virtio_blk_api::{Coords, Observe, Supply, Wire, OBSERVE, SUPPLY};
use wire::Message;

#[test]
fn activation_coordinates_keep_capacity_and_exact_wire_bytes() {
    let value = Coords {
        bar_pages: 2, common: 0x40, notify: 0x80, multiplier: 4, device: 0x100,
        iova: 0x1_2345_6000, bar: env::PieToken::mint(17), bytes: 8192,
    };
    assert_eq!(Coords::LEN, 48);
    let mut bytes = Coords::EMPTY;
    let len = value.store(&mut bytes).unwrap();
    assert_eq!(len, 44);
    let golden = [
        2, 0, 0, 0, 0x40, 0, 0, 0, 0x80, 0, 0, 0, 4, 0, 0, 0,
        0, 1, 0, 0, 0, 0x60, 0x45, 0x23, 1, 0, 0, 0,
        17, 0, 0, 0, 0, 0, 0, 0, 0, 0x20, 0, 0, 0, 0, 0, 0,
    ];
    assert_eq!(&bytes[..len], &golden);
    assert_eq!(Coords::fetch(&bytes[..len]), Some(value));
    assert!(Coords::fetch(&bytes[..len-1]).is_none());
    let mut extended = bytes[..len].to_vec();
    extended.push(0);
    assert!(Coords::fetch(&extended).is_none());
    assert!(value.store(&mut [0; 43]).is_none());
}

#[test]
fn binding_operations_require_exact_frame_and_operation_tag() {
    let observe = Observe { op: OBSERVE, hub: env::TaskId::new(3), serial: 7, back: env::PieToken::mint(9) };
    let supply = Supply { op: SUPPLY, hub: observe.hub, serial: observe.serial,
        source: env::PieToken::mint(12), access: 3, back: observe.back };
    assert_eq!((Observe::LEN, Supply::LEN), (32, 40));
    for request in [Wire::Observe(observe), Wire::Supply(supply)] {
        let mut bytes = Wire::EMPTY;
        let len = request.store(&mut bytes).unwrap();
        assert_eq!(Wire::fetch(&bytes[..len]), Some(request));
        assert_eq!(request.identity(), hub_api::BindingId { hub: observe.hub, serial: 7 });
        assert!(Wire::fetch(&bytes[..len-1]).is_none());
        let mut extended = bytes[..len].to_vec();
        extended.push(0);
        assert!(Wire::fetch(&extended).is_none());
    }
    let mut rejected = Wire::EMPTY;
    assert!(Wire::Observe(Observe { op: SUPPLY, ..observe }).store(&mut rejected).is_none());
    assert!(Wire::fetch(&[99; 40]).is_none());
}
