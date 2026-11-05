#![no_std]
//! Resident device Driver with a declared coordination endpoint.
//! It remains outside the execution Team and acquires exact BAR grants from PCI.

use unit::{Dependency, Ending, Identity, Ready, UnitFile};

pub static PROGRAM: UnitFile = UnitFile {
    name: "virtio-blk",
    loader: unit::LoaderConfig::supervisor("virtio-blk"),
    operator: unit::OperatorConfig {
        namespace: &[unit::operator::Namespace {
            key: system_api::namespace::SERVICE,
            placement: unit::operator::Placement::Shared,
            path: "device",
            scope: unit::operator::Scope::Names(&["coord"]),
            ops: system_api::operator::Ops::BIND,
            permit: Some(system_api::operator::Permit::Public),
            source_mark: Some(hub_api::coordination::call::ENTRY.get()),
            bindings: &[],
        }],
    },
    identity: Identity::DEFAULT,
    control: unit::ControlConfig {
        supervision: None,
        after: Some(&[
            Dependency::Bootstrap(unit::Bootstrap::Operator),
            Dependency::Bootstrap(unit::Bootstrap::Identity),
            Dependency::Unit("hub"),
            Dependency::Unit("pci"),
            Dependency::Unit("iommu"),
        ]),
        ready: Ready::Signal(unit::READY_MARK),
        ending: Some(Ending::Resident),
        construction: &[],
        retirement: &[unit::Retirement { provider: "hub", namespace: 0, face: "retire" }],
    },
    supply: unit::SupplyConfig { bindings: &[] },
};

pub const BUILD: unit::ImageBuild = unit::ImageBuild {
    image: "virtio-blk",
    manifest: "server/Cargo.toml",
    package: "virtio-blk-server",
    bin: "virtio-blk",
    features: &[],
    target: "riscv64gc-unknown-none-elf",
    profile: "release",
    default_features: true,
};
