# VirtIO block Driver

Repository: [shuyra-arche/driver-virtio-block](https://github.com/shuyra-arche/driver-virtio-block).

This repository contains the VirtIO block runtime extracted from
[`sqware` at `94e114614bec53424ac55e640fc80cf19a3e7107`](https://github.com/anreonyr/sqware/tree/94e114614bec53424ac55e640fc80cf19a3e7107/programs/src/driver/virtio).

The workspace separates the public binding messages and `Coords` (`api`),
the resident UnitFile/build coordinates (`declaration`), and live protocol,
coordination and BAR recovery (`server`). The `virtio-blk` binary runs the real
resident service. PCI configuration inspection is consumed through the PCI
provider's public `pci_api::inspection` contract; IOMMU protocol types use their
existing provider.

The provider-owned [unit.toml](unit.toml) lets Vanllia discover the existing
Cargo binary and select Hub, PCI and IOMMU as startup prerequisites. System
completes Operator and Identity bootstrap internally. The first namespace
capability publishes `device/coord`; the driver retains Hub retirement policy.
There are no static hardware supplies: PCI grants the exact BAR, while the
execution task supplies DMA memory through the binding protocol. See
[intake validation and interface requirements](INTAKE.md).

The original peer/owner checks, exact BAR bounds and permissions, monotonic
binding identities, asynchronous retry/rollback and RISC-V device fences are
retained. Register setup/queue/notification/reset mechanics remain private to
the server; no public safe API accepting arbitrary raw MMIO addresses is added.

On a Linux x86_64 host with the pinned Rust toolchain and RV64 target installed:

```sh
cargo fetch --locked
cargo build --locked --offline -p virtio-blk-server --bin virtio-blk --release
cargo test --locked --offline -p virtio-blk-api --target x86_64-unknown-linux-gnu
cargo check --locked --offline -p virtio-blk-declaration --target x86_64-unknown-linux-gnu
rustc --test --edition=2024 --target x86_64-unknown-linux-gnu server/tests/capabilities.rs -o /tmp/virtio-blk-capabilities
/tmp/virtio-blk-capabilities
cargo test --locked --manifest-path tests/unit-document/Cargo.toml --target x86_64-unknown-linux-gnu
```

No old scenario/controller or test suite is carried over. There is no new general
read/write block service; fixed-sector fixture workloads remain historical
material. Compilation and interface checks do not prove live DMA, reset or
recovery behavior. The resident service discovers devices and coordinates
bindings; it does not run the private queue execution helpers or issue block
requests. The opt-in [discovery smoke Unit](tests/discovery/README.md) passed a
one-core QEMU run with authenticated PCI discovery, IOMMU domain establishment,
Hub enrollment and native clean retirement. Removing its block device correctly
fails the discovery verdict. These checks do not prove real block I/O, queue
DMA, data integrity or SMP. New SDK compatibility uses Vanllia's native Cargo
overlay; PCI and IOMMU dependencies pin their accepted public intake commits.
See [provenance and scope](PROVENANCE.md) and [source identities](SOURCE_FILES.json).

The initial `main` baseline contains only a README. The migration runtime is
reviewed in its child Draft PR; it is not hidden in repository initialization.

The frozen source root and selected subtree contain no license file. This
setup does not invent a license or claim a new license grant. Source headers,
attribution and existing history will be retained with the migration.
