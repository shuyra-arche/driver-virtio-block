# Runtime origin and boundary

The runtime comes from `anreonyr/sqware` at the frozen full commit
`94e114614bec53424ac55e640fc80cf19a3e7107`. `SOURCE_FILES.json` records the
original paths, Git blob identities, hashes and exact namespace substitutions.
Original module comments and safety checks remain. No legacy checkout is
required to build the resulting workspace.

The original VirtIO history remains available without rewriting or deleting
the old repository. Useful checkpoints retained in the source record include
`0db30aae` (hardware coordination ownership), `5d2f3de6` (independent binding
progress and exact BAR roots), `d1abfcad` (orphan retirement), and `2c17c387`
(declaration/composition separation). The new repository has a README-only
initial baseline; the runtime belongs to its child migration PR.

The frozen root and selected source subtrees contain no license file. The
migration preserves existing headers and attribution and does not invent a
license or assert redistribution clearance. Third-party crates remain pinned
dependencies with their own upstream notices; no firmware binary is included.

## Ownership

- `api` owns the existing binding messages, marks and `Coords` frame. Its frozen
  SDK capacity is 48 bytes; the encoded payload is 44 bytes, without padding.
- `declaration` owns the existing UnitFile and standalone build coordinates.
- `server` owns capability parsing, real virtio-pci register negotiation,
  private queue setup/notification, per-device coordination, exact BAR grant checks,
  stopped-target/reset/drain and mapping retirement.
- Driver context and authenticated peers use their existing public providers.
  Only the missing lookup adapter and RISC-V device-order fence stay private.
  The private `DomainId` is a reference to an IOMMU-issued engine/key pair;
  this Driver does not issue IOMMU identities.

The server exposes its resident entry only. Frozen raw-address MMIO helpers
remain private rather than becoming new public safe APIs without a mapping
and lifetime contract. PCI/IOMMU endpoint coordinates use existing provider
constants with the same frozen string values.

The existing coordinate contract has one owner here. Hub mark functions
are consumed directly; the unused legacy `HubDma` wrapper is not copied.

## Deliberate exclusions

No old tests, controllers, scenes or deployment assembly are migrated. Inspection
of `programs/src/harness/device/work.rs` and `layout.rs` found controller modes,
fixed sector-zero reads, a sentinel/disk-pattern oracle, and round-indexed ring
storage without a general ring bounds/ownership contract. Those sources remain
in the historical repository. Promoting them would require a new API design.

This migration preserves the existing live device setup and coordination APIs;
it does not introduce a general read/write block service or claim that the
old fixture workload has become a production service. No QEMU, DMA transfer,
device recovery or SMP execution is validated by compilation or host checks.
