#[cfg(test)]
mod tests {
    use unit::{document as d, record};
    use virtio_blk_declaration::{BUILD, PROGRAM};

    fn document() -> d::Unit<String> {
        toml_edit::de::from_str(include_str!("../../unit.toml")).unwrap()
    }

    fn assert_runtime_contract<S: AsRef<str>>(unit: &d::Unit<S>) {
        unit.validate().unwrap();
        assert_eq!(unit.id.as_ref(), PROGRAM.name);
        assert_eq!(unit.loader.image.as_ref(), BUILD.image);
        assert_eq!(unit.loader.space, d::Space::Supervisor);
        assert!(unit.loader.args.is_empty());
        assert_eq!(unit.loader.stack, 0);
        assert_eq!(unit.operator.namespace.len(), 1);
        let namespace = &unit.operator.namespace[0];
        assert_eq!(namespace.key.as_ref(), "service");
        assert_eq!(namespace.placement, d::Placement::Shared);
        assert_eq!(namespace.path.as_ref(), "device");
        match &namespace.scope {
            d::Scope::Names(names) => {
                assert_eq!(
                    names.iter().map(AsRef::as_ref).collect::<Vec<_>>(),
                    ["coord"]
                );
            }
            _ => panic!("coord publication must be limited to its declared name"),
        }
        assert_eq!(namespace.ops, PROGRAM.operator.namespace[0].ops.bits());
        assert_eq!(namespace.permit, Some(d::Permit::Public));
        assert_eq!(
            namespace.source_mark,
            Some(hub_api::coordination::call::ENTRY.get())
        );
        assert!(namespace.bindings.is_empty());
        assert!(unit.identity.aliases.is_empty());
        assert!(unit.identity.grants.is_empty());
        assert_eq!(unit.control.creation, d::Creation::Startup);
        let dependencies = ["hub", "pci", "iommu"];
        assert_eq!(
            unit.control
                .requires
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<_>>(),
            dependencies
        );
        assert_eq!(
            unit.control
                .after
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<_>>(),
            dependencies
        );
        assert_eq!(unit.control.ready, d::Ready::Signal(unit::READY_MARK.get()));
        assert_eq!(unit.control.ending, d::Ending::Resident);
        assert!(unit.control.supervision.is_none());
        assert!(unit.control.construction.is_empty());
        assert!(unit.control.groups.is_empty());
        assert!(!unit.control.guarded);
        assert_eq!(unit.control.retirement.len(), 1);
        let retirement = &unit.control.retirement[0];
        assert_eq!(retirement.provider.as_ref(), "hub");
        assert_eq!(retirement.namespace, 0);
        assert_eq!(retirement.face.as_ref(), "retire");
        assert!(unit.supply.bindings.is_empty());
    }

    #[test]
    fn startup_dependencies_and_authority_survive_native_record_encoding() {
        let unit = document();
        assert_runtime_contract(&unit);
        let bytes = record::encode(&unit).unwrap();
        let decoded = record::decode(&bytes).unwrap();
        assert_runtime_contract(&decoded);
    }

    #[test]
    fn discovered_image_uses_the_existing_resident_entry() {
        let manifest: toml_edit::DocumentMut =
            include_str!("../../server/Cargo.toml").parse().unwrap();
        assert_eq!(manifest["package"]["name"].as_str(), Some(BUILD.package));
        assert_eq!(BUILD.manifest, "server/Cargo.toml");
        let bins = manifest["bin"].as_array_of_tables().unwrap();
        assert_eq!(bins.len(), 1);
        let bin = bins.get(0).unwrap();
        assert_eq!(bin["name"].as_str(), Some(document().loader.image.as_str()));
        let entry = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../server")
            .join(bin["path"].as_str().unwrap());
        assert!(entry.is_file());
    }
}
