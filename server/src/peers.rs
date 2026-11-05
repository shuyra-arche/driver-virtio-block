//! Shared authenticated endpoints used by every physical device coordinator.

use env::TaskId;

use iommu_api as iommu;
use pci_api as pci;
use driver_support::peer::Peer;

pub struct Peers {
    pub hub: TaskId,
    pub pci: Peer<pci::Call>,
    pub iommu: Peer<iommu::Call>,
}
