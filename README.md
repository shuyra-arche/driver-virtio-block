# VirtIO block Unit

This provider discovers VirtIO PCI block devices and coordinates authenticated
bindings. The resident service does not issue block requests or call its private
queue-execution helpers; a general read/write service is not implemented here.

## Layout and boundaries

- `api/`: binding messages and coordinates.
- `server/`: private protocol, coordination, capability and recovery code;
  the `virtio-blk` binary calls `serve()`.
- `declaration/`: compatibility Rust policy/build declaration.
- `unit.toml`: startup after Hub, PCI and IOMMU.
- `tests/unit-document/`: host policy/record checks.
- `tests/discovery/`: optional native discovery probe.

PCI inspection and placement must authenticate as the same task. IOMMU is a
separate authenticated peer. PCI grants the exact BAR capability; execution
supplies DMA memory dynamically. There are no static BAR or DMA-source supplies.
Teardown preserves IOMMU Invalidate → PCI Disable → device reset/readback →
IOMMU Quiesce → IOMMU Release → BAR detach → PCI Release.
Failure retains rollback state. Raw-address MMIO helpers remain private.

## Build and test

Install Rust through rustup; the checkout pins nightly-2026-09-04 and RV64.
Uncached dependencies require network access.

```sh
cargo build --locked -p virtio-blk-server --bin virtio-blk --target riscv64gc-unknown-none-elf --release
cargo test --locked -p virtio-blk-api --target x86_64-unknown-linux-gnu
cargo check --locked -p virtio-blk-declaration --target x86_64-unknown-linux-gnu
rustc --test --edition=2024 --target x86_64-unknown-linux-gnu server/tests/capabilities.rs -o /tmp/virtio-blk-capabilities
/tmp/virtio-blk-capabilities
cargo test --locked --manifest-path tests/unit-document/Cargo.toml --target x86_64-unknown-linux-gnu
```

The [discovery probe](tests/discovery/README.md) uses
[Vanllia](https://github.com/shuyra-arche/vanllia) with explicit QEMU hardware.
Host compilation and discovery checks do not prove block I/O, queue DMA,
data integrity, reset/recovery or SMP behavior.

See [source attribution](PROVENANCE.md) and the
[organization contribution guide](https://github.com/shuyra-arche/vanllia/blob/main/CONTRIBUTING.md).
