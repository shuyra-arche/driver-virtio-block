# VirtIO block Unit intake

This extends Draft PR #1 from `65884d635713806d3f20ad42f8337cfe2bed08a2`.
The existing runtime and its provenance mappings are retained. New files add the
provider-owned Unit document, focused document checks and an opt-in discovery
smoke Unit. No old test suite or kernel cluster is added.

## Entry, ownership and supply

`server/src/main.rs` already calls `virtio_blk_server::serve()` and links through
Execution's published linker script. Vanllia discovers the Cargo bin named
`virtio-blk`; no second runtime entry or central image registration is needed.
The Unit keeps supervisor space, readiness mark, Hub retirement and namespace
index 0 from the existing Rust declaration. Its startup prerequisites are Hub,
PCI and IOMMU. System owns Operator and Identity bootstrap.

The namespace capability permits publication of `device/coord` in the shared
service namespace. The source mark is `hub_api::coordination::call::ENTRY`.
System's public control State face supplies authenticated publisher identities;
the runtime checks the owner and mark of each peer before use.

Static supply bindings are empty deliberately. ECAM and aperture belong to PCI;
IOMMU region and retention belong to IOMMU. PCI Place grants the coordinator an
exact BAR capability with the expected PCI owner, mark, permissions and size.
The coordinator grants a narrower BAR view to execution. An execution task uses
the binding `Supply` operation to provide its DMA source and access, which flow
to IOMMU Acquire. A failed or retrying binding retains the original rollback
state and retirement order; this intake changes no runtime ownership logic.

## Required public interfaces

| Provider | Contract consumed by this runtime |
| --- | --- |
| PCI inspection | `pci_api::inspection::{Call, Scan, Page}`, `pci/config`, mark `inspection::ENTRY`, reply mark `inspection::BACK`. Scan returns bounded 256-byte configuration snapshots, requester, next cursor and EOF. |
| PCI placement | `pci_api::{Call, Wire, Place, Toggle}`, `pci/entry`, mark `ENTRY`. Place returns BAR token and page count to the actual coordinator caller. Enable, Disable and Release retain their separate effects. |
| IOMMU | `iommu_api::{Call, Wire, Establish, Acquire, Step}`, `iommu/entry`, mark `ENTRY`. Establish returns a nonzero key and the authenticated IOMMU engine task. Acquire uses that domain, Hub/serial binding identity, execution task, source and access. |
| Hub | `hub/dma` enrolls devices; `device/coord` handles Prepare/Release. Binding handoff and retirement keep the complete Hub/serial identity. |

PCI inspection and placement must be published by the same task; the runtime
rejects different owners. After the parallel provider intakes merged, this
workspace's manifest and lock pin their accepted heads:

- PCI: `b2c2e5d7a5fbd0b1572273dbbffb56bd91b15009`.
- IOMMU: `3d210b4da522ee0c7427265ce0a0d8946e124b21`.

Both API directories are byte-identical to the previous pins. No wire or
coordinator state-machine adaptation is required. Both providers now include
their own Unit documents and static hardware supplies.

Teardown remains IOMMU Invalidate → PCI Disable → device reset/readback →
IOMMU Quiesce → IOMMU Release → BAR detach → PCI Release. The existing exact
capability and authenticated peer checks are unchanged.

## Validation on 2026-11-05

The managed environment used `nightly-2026-10-09`. Vanllia CLI at
`image-builder@ea9d4f680fee37232b85dfd69f77935c00383170` generated native Cargo
overlays and isolated lockfiles in a separate `/workspace/virtio-intake`
project. No shared Vanllia or Driver branch was modified. Selected SDK sources:

| Source | Revision |
| --- | --- |
| env | `ee9357070996e2e559095904ff137a289a4ca156` |
| wire | `08293d2de463901f69ac66b62e9cd3a6ab5629c3` |
| mold | `8e8b1c2bdf188cf01494a3a93398cc77cd12b3e6` |
| unit | `ad50b803c3bd5cd2ef90010d80badf23551e88d5` |
| System | `ca0946e4267db494d35acf6c07dd341bb90ef111` |
| Driver shared/support | `f1b56be5a3fa472b6aa301bbfba0ffde878a5ae7` |

The selected env facade resolves Kernel API definitions at
`a9986c82127cf25183bf5c30e7d42e4a9376b734`. Locked offline metadata confirmed
one instance each of env, wire, mold, unit, System API/client, Driver
shared/support, PCI API, IOMMU API and the Kernel env/wire/macros owner packages.
The CLI generates patches for all discovered providers, so Cargo also reports
unused optional/test patches. Hub/Router retain their historical URL pins;
renaming repositories is not part of this intake.

Passed with the selected SDK overlay and isolated locks:

- RV64 release build of the real `virtio-blk` binary (`--locked --offline`).
- Two existing API wire-contract checks, including the 48-byte coordinate
  capacity with its exact 44-byte encoded payload.
- Two new Unit document checks: native encode/decode preserves startup
  dependencies, publication authority, readiness, retirement and supply policy;
  the image resolves to the existing Cargo binary and source entry.
- Two existing pure capability-parser checks compiled directly on the host.

The two document checks also passed with the retained standalone Cargo
graph. `git diff --check` passed. The server retains 29 existing unused-code and
reexport warnings. The new SDK binary is a 352616-byte RV64 ET_EXEC ELF with
entry `0x1d7a4`; SHA-256
`468f9f0cdb7f59fbf399b4f95f8961f1139cbe059f54243c73e18d2a183af85f`.
This identifies this artifact, not a byte-reproducibility guarantee.

## Accepted-dependency composition and guest smoke

With the accepted PCI/IOMMU heads, `vanllia check` and `vanllia plan` pass. The
minimal resident closure is System → Hub → IOMMU/PCI → VirtIO block. The opt-in
`test.virtio-blk-discovery` transient Unit extends that closure only for the
focused discovery check; production selection does not start the test.

The first complete build exposed a System metadata omission: System already
registers `memory/retention`, but `system@ca0946e` did not list it among its
runtime resources. Packaging correctly refused the IOMMU supply. The smoke
uses the existing public System companion fix at
`b22bbfb0d37e48e62be9c8f2e365598f9e30906a`, selected in a separate worktree;
no System or shared composition branch was edited here. Parent intake must
include that declaration fix. Kernel is the current product pin
`44e33a6ee28d86eeeabe61c225f4176c5ed64c59`; the SDK facades resolve through
the product's native overlay. Shared SDK pins remain unchanged in this
provider's standalone manifest.

`vanllia build --unit test.virtio-blk-discovery` produced a complete six-Unit
artifact. QEMU 10.0.13 used one core, 256 MiB, `virt,iommu-sys=on` and one
read-only 2 MiB scratch disk on modern `virtio-blk-pci` with
`iommu_platform=on,disable-legacy=on`. Firmware was an explicitly hashed local
OpenSBI input and is not committed. The complete artifact identities are in
[validation/discovery/result.json](validation/discovery/result.json).

- Initrd: 1941504 bytes, SHA-256
  `e8f2e1b8d71274692d3970880507e7c452bedb4420e0a1a79719badd4b21c1bd`.
- VirtIO binary: 352672 bytes, SHA-256
  `77b01a2cdda7625e2787cc79e485c00a1f8a153e6161b2d96614add673987bd7`.
- Guest logs show IOMMU domain 1 established for coordinator task 7/requester 8,
  then `virtio-blk: serving hub=4 pci=6 devices=1`.
- The test authenticates System State and Hub's publisher/list face and requires
  exactly `virtio-blk-pci-1` in the Hub class. Public
  `vanllia::testing::verify(..., Expectation::Clean)` passed: native test exit 0,
  System completion and acknowledged retirement of every task. VM exit was 0.
- A second run of the same artifact without the block device exited the test
  with reason 12, and the native clean verifier rejected it. VM exit 0 alone
  cannot pass this check.

The preserved serial evidence contains every native System frame, with only
OpenSBI's prose prefix removed and UART CRLF normalized. The current generic
runner cannot attach the extra hardware, so this test is opt-in, with explicit
QEMU inputs documented in [tests/discovery/README.md](tests/discovery/README.md).
Independent review checked the pin changes, authenticated test and its bounded
failure path; the existing host tests and locked offline standalone RV64 build
also passed after the dependency update.

These guest results prove the discovery/enrollment path and observed domain
establishment. They do not exercise block requests, queue DMA, data integrity,
binding reset/recovery, DMA isolation or SMP. The resident coordinator does not
call the private queue execution helpers; real I/O still needs an execution
consumer with vrings and block requests. Driver/Vanllia gitlink intake and
merge ordering remain owned by the parent integration task.
