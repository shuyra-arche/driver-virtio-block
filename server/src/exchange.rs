//! The one in-flight peer exchange owned by a binding operation.

use ipc::rpc::request::Pending;

use iommu_api as iommu;
use pci_api as pci;

pub enum Exchange {
    Pci(Pending<pci::Call>),
    Iommu(Pending<iommu::Call>),
}
