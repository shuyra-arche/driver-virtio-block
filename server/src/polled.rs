use iommu_api as iommu;
use pci_api as pci;

pub(super) enum Polled {
    Pci(Result<Option<pci::Said>, ()>),
    Iommu(Result<Option<iommu::Said>, ()>),
    Empty,
}
