#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GICH hypervisor control register"]
    pub gich_hcr: crate::Reg<gich_hcr::GICH_HCR_SPEC>,
    #[doc = "0x04 - GICH VGIC type register"]
    pub gich_vtr: crate::Reg<gich_vtr::GICH_VTR_SPEC>,
    #[doc = "0x08 - GICH virtual machine control register"]
    pub gich_vmcr: crate::Reg<gich_vmcr::GICH_VMCR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - GICH maintenance interrupt status register"]
    pub gich_misr: crate::Reg<gich_misr::GICH_MISR_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x20 - GICH end of interrupt status register"]
    pub gich_eisr0: crate::Reg<gich_eisr0::GICH_EISR0_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x30 - GICH empty list status register"]
    pub gich_elsr0: crate::Reg<gich_elsr0::GICH_ELSR0_SPEC>,
    _reserved6: [u8; 0xbc],
    #[doc = "0xf0 - GICH active priority register"]
    pub gich_apr0: crate::Reg<gich_apr0::GICH_APR0_SPEC>,
    _reserved7: [u8; 0x0c],
    #[doc = "0x100 - GICH list register 0"]
    pub gich_lr0: crate::Reg<gich_lr0::GICH_LR0_SPEC>,
    #[doc = "0x104 - GICH list register 1"]
    pub gich_lr1: crate::Reg<gich_lr1::GICH_LR1_SPEC>,
    #[doc = "0x108 - GICH list register 2"]
    pub gich_lr2: crate::Reg<gich_lr2::GICH_LR2_SPEC>,
    #[doc = "0x10c - GICH list register 3"]
    pub gich_lr3: crate::Reg<gich_lr3::GICH_LR3_SPEC>,
}
#[doc = "GICH_HCR register accessor: an alias for `Reg<GICH_HCR_SPEC>`"]
pub type GICH_HCR = crate::Reg<gich_hcr::GICH_HCR_SPEC>;
#[doc = "GICH hypervisor control register"]
pub mod gich_hcr;
#[doc = "GICH_VTR register accessor: an alias for `Reg<GICH_VTR_SPEC>`"]
pub type GICH_VTR = crate::Reg<gich_vtr::GICH_VTR_SPEC>;
#[doc = "GICH VGIC type register"]
pub mod gich_vtr;
#[doc = "GICH_VMCR register accessor: an alias for `Reg<GICH_VMCR_SPEC>`"]
pub type GICH_VMCR = crate::Reg<gich_vmcr::GICH_VMCR_SPEC>;
#[doc = "GICH virtual machine control register"]
pub mod gich_vmcr;
#[doc = "GICH_MISR register accessor: an alias for `Reg<GICH_MISR_SPEC>`"]
pub type GICH_MISR = crate::Reg<gich_misr::GICH_MISR_SPEC>;
#[doc = "GICH maintenance interrupt status register"]
pub mod gich_misr;
#[doc = "GICH_EISR0 register accessor: an alias for `Reg<GICH_EISR0_SPEC>`"]
pub type GICH_EISR0 = crate::Reg<gich_eisr0::GICH_EISR0_SPEC>;
#[doc = "GICH end of interrupt status register"]
pub mod gich_eisr0;
#[doc = "GICH_ELSR0 register accessor: an alias for `Reg<GICH_ELSR0_SPEC>`"]
pub type GICH_ELSR0 = crate::Reg<gich_elsr0::GICH_ELSR0_SPEC>;
#[doc = "GICH empty list status register"]
pub mod gich_elsr0;
#[doc = "GICH_APR0 register accessor: an alias for `Reg<GICH_APR0_SPEC>`"]
pub type GICH_APR0 = crate::Reg<gich_apr0::GICH_APR0_SPEC>;
#[doc = "GICH active priority register"]
pub mod gich_apr0;
#[doc = "GICH_LR0 register accessor: an alias for `Reg<GICH_LR0_SPEC>`"]
pub type GICH_LR0 = crate::Reg<gich_lr0::GICH_LR0_SPEC>;
#[doc = "GICH list register 0"]
pub mod gich_lr0;
#[doc = "GICH_LR1 register accessor: an alias for `Reg<GICH_LR1_SPEC>`"]
pub type GICH_LR1 = crate::Reg<gich_lr1::GICH_LR1_SPEC>;
#[doc = "GICH list register 1"]
pub mod gich_lr1;
#[doc = "GICH_LR2 register accessor: an alias for `Reg<GICH_LR2_SPEC>`"]
pub type GICH_LR2 = crate::Reg<gich_lr2::GICH_LR2_SPEC>;
#[doc = "GICH list register 2"]
pub mod gich_lr2;
#[doc = "GICH_LR3 register accessor: an alias for `Reg<GICH_LR3_SPEC>`"]
pub type GICH_LR3 = crate::Reg<gich_lr3::GICH_LR3_SPEC>;
#[doc = "GICH list register 3"]
pub mod gich_lr3;
