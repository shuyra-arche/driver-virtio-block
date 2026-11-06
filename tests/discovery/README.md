# Discovery smoke Unit

Select `test.virtio-blk-discovery` in a Vanllia composition that includes this
provider, the accepted PCI/IOMMU providers, Hub and System's declared
`memory/retention` runtime resource. The Unit keeps System active while the
resident coordinator finishes discovery. It authenticates System State and the
Hub publisher/list endpoint, then requires exactly `virtio-blk-pci-1` to appear
in Hub's `virtio-blk` class within five seconds. It issues no claim, block
request or queue/DMA operation.

```sh
vanllia check
vanllia plan --unit test.virtio-blk-discovery
vanllia build --unit test.virtio-blk-discovery
```

Use the resulting receipt's verified firmware, Kernel and initrd with QEMU:
`-machine virt,iommu-sys=on -m 256M -smp 1`, and attach one scratch raw disk
read-only with `-drive file=SCRATCH,format=raw,if=none,readonly=on,id=block`
and `-device virtio-blk-pci,drive=block,iommu_platform=on,disable-legacy=on`.
The checked fixture was a zero-filled 2 MiB file. The current generic Vanllia
runner has no extra-device input, so this hardware-specific Unit is opt-in and
has no default `test.toml` registration.

Judge the complete serial stream with public
`vanllia::testing::verify(serial, "test.virtio-blk-discovery", Expectation::Clean)`.
This requires the test's native zero exit, System completion and every task's
acknowledged retirement; a diagnostic banner alone cannot pass. Keep a bounded
host timeout. Without the attached block device, the five-second lookup ends
with reason 12 and the clean verdict must reject it.

Keep each run's complete serial stream and artifact identities with its CI or
release artifacts. This check covers authenticated discovery and Hub enrollment;
it does not validate block I/O, queue DMA or data integrity.
