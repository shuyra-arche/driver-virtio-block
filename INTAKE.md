# VirtIO block Unit intake

This extends Draft PR #1 from `65884d635713806d3f20ad42f8337cfe2bed08a2`.
The existing runtime and its provenance mappings are retained. The new files are
the provider-owned Unit document and focused document checks, not another source
extraction. No old test suite or kernel cluster is added.

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
rejects different owners. The tested PCI API is
`driver-pci@bc0da12a1bafbf684179301853b5709a1ddfeaf9` (its current Draft runtime
checkpoint). IOMMU API/declaration were selected from
`iommu@9bb54099d579e62d2c844ca25d7e5abab8c75f91`; the retained standalone
manifest still pins its existing public API revision. No newer parallel server
or Unit document is assumed complete.

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

The two new document checks also passed with the retained standalone Cargo
graph. `git diff --check` passed. The server retains 29 existing unused-code and
reexport warnings. The new SDK binary is a 352616-byte RV64 ET_EXEC ELF with
entry `0x1d7a4`; SHA-256
`468f9f0cdb7f59fbf399b4f95f8961f1139cbe059f54243c73e18d2a183af85f`.
This identifies this artifact, not a byte-reproducibility guarantee.

`vanllia check` on the isolated minimal composition currently fails with
`Unit virtio-blk references unknown Unit pci`: the separately checked out PCI
and IOMMU candidate checkpoints have no `unit.toml` yet. Both parallel provider
intakes must publish their Unit documents and hardware supplies before the
composition can be checked, packaged and booted. Driver/Vanllia gitlink intake
and merge ordering belong to the parent integration task.

No guest execution, authenticated discovery, actual MMIO, block request,
queue DMA, data-integrity, reset/recovery, IOMMU enforcement or SMP result is
claimed. The resident coordinator does not call the private queue execution
helpers; a real execution consumer must provide vrings and issue block requests
to validate I/O and DMA. Starting the coordination service alone is insufficient.
