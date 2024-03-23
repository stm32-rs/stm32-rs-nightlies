#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gich_hcr: GICH_HCR,
    gich_vtr: GICH_VTR,
    gich_vmcr: GICH_VMCR,
    _reserved3: [u8; 0x04],
    gich_misr: GICH_MISR,
    _reserved4: [u8; 0x0c],
    gich_eisr0: GICH_EISR0,
    _reserved5: [u8; 0x0c],
    gich_elsr0: GICH_ELSR0,
    _reserved6: [u8; 0xbc],
    gich_apr0: GICH_APR0,
    _reserved7: [u8; 0x0c],
    gich_lr0: GICH_LR0,
    gich_lr1: GICH_LR1,
    gich_lr2: GICH_LR2,
    gich_lr3: GICH_LR3,
}
impl RegisterBlock {
    #[doc = "0x00 - GICH hypervisor control register"]
    #[inline(always)]
    pub const fn gich_hcr(&self) -> &GICH_HCR {
        &self.gich_hcr
    }
    #[doc = "0x04 - GICH VGIC type register"]
    #[inline(always)]
    pub const fn gich_vtr(&self) -> &GICH_VTR {
        &self.gich_vtr
    }
    #[doc = "0x08 - GICH virtual machine control register"]
    #[inline(always)]
    pub const fn gich_vmcr(&self) -> &GICH_VMCR {
        &self.gich_vmcr
    }
    #[doc = "0x10 - GICH maintenance interrupt status register"]
    #[inline(always)]
    pub const fn gich_misr(&self) -> &GICH_MISR {
        &self.gich_misr
    }
    #[doc = "0x20 - GICH end of interrupt status register"]
    #[inline(always)]
    pub const fn gich_eisr0(&self) -> &GICH_EISR0 {
        &self.gich_eisr0
    }
    #[doc = "0x30 - GICH empty list status register"]
    #[inline(always)]
    pub const fn gich_elsr0(&self) -> &GICH_ELSR0 {
        &self.gich_elsr0
    }
    #[doc = "0xf0 - GICH active priority register"]
    #[inline(always)]
    pub const fn gich_apr0(&self) -> &GICH_APR0 {
        &self.gich_apr0
    }
    #[doc = "0x100 - GICH list register 0"]
    #[inline(always)]
    pub const fn gich_lr0(&self) -> &GICH_LR0 {
        &self.gich_lr0
    }
    #[doc = "0x104 - GICH list register 1"]
    #[inline(always)]
    pub const fn gich_lr1(&self) -> &GICH_LR1 {
        &self.gich_lr1
    }
    #[doc = "0x108 - GICH list register 2"]
    #[inline(always)]
    pub const fn gich_lr2(&self) -> &GICH_LR2 {
        &self.gich_lr2
    }
    #[doc = "0x10c - GICH list register 3"]
    #[inline(always)]
    pub const fn gich_lr3(&self) -> &GICH_LR3 {
        &self.gich_lr3
    }
}
#[doc = "GICH_HCR (rw) register accessor: GICH hypervisor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gich_hcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gich_hcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gich_hcr`]
module"]
pub type GICH_HCR = crate::Reg<gich_hcr::GICH_HCRrs>;
#[doc = "GICH hypervisor control register"]
pub mod gich_hcr;
#[doc = "GICH_VTR (r) register accessor: GICH VGIC type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gich_vtr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gich_vtr`]
module"]
pub type GICH_VTR = crate::Reg<gich_vtr::GICH_VTRrs>;
#[doc = "GICH VGIC type register"]
pub mod gich_vtr;
#[doc = "GICH_VMCR (rw) register accessor: GICH virtual machine control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gich_vmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gich_vmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gich_vmcr`]
module"]
pub type GICH_VMCR = crate::Reg<gich_vmcr::GICH_VMCRrs>;
#[doc = "GICH virtual machine control register"]
pub mod gich_vmcr;
#[doc = "GICH_MISR (r) register accessor: GICH maintenance interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gich_misr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gich_misr`]
module"]
pub type GICH_MISR = crate::Reg<gich_misr::GICH_MISRrs>;
#[doc = "GICH maintenance interrupt status register"]
pub mod gich_misr;
#[doc = "GICH_EISR0 (r) register accessor: GICH end of interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gich_eisr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gich_eisr0`]
module"]
pub type GICH_EISR0 = crate::Reg<gich_eisr0::GICH_EISR0rs>;
#[doc = "GICH end of interrupt status register"]
pub mod gich_eisr0;
#[doc = "GICH_ELSR0 (r) register accessor: GICH empty list status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gich_elsr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gich_elsr0`]
module"]
pub type GICH_ELSR0 = crate::Reg<gich_elsr0::GICH_ELSR0rs>;
#[doc = "GICH empty list status register"]
pub mod gich_elsr0;
#[doc = "GICH_APR0 (rw) register accessor: GICH active priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gich_apr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gich_apr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gich_apr0`]
module"]
pub type GICH_APR0 = crate::Reg<gich_apr0::GICH_APR0rs>;
#[doc = "GICH active priority register"]
pub mod gich_apr0;
#[doc = "GICH_LR0 (rw) register accessor: GICH list register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gich_lr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gich_lr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gich_lr0`]
module"]
pub type GICH_LR0 = crate::Reg<gich_lr0::GICH_LR0rs>;
#[doc = "GICH list register 0"]
pub mod gich_lr0;
#[doc = "GICH_LR1 (rw) register accessor: GICH list register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gich_lr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gich_lr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gich_lr1`]
module"]
pub type GICH_LR1 = crate::Reg<gich_lr1::GICH_LR1rs>;
#[doc = "GICH list register 1"]
pub mod gich_lr1;
#[doc = "GICH_LR2 (rw) register accessor: GICH list register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gich_lr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gich_lr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gich_lr2`]
module"]
pub type GICH_LR2 = crate::Reg<gich_lr2::GICH_LR2rs>;
#[doc = "GICH list register 2"]
pub mod gich_lr2;
#[doc = "GICH_LR3 (rw) register accessor: GICH list register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gich_lr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gich_lr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gich_lr3`]
module"]
pub type GICH_LR3 = crate::Reg<gich_lr3::GICH_LR3rs>;
#[doc = "GICH list register 3"]
pub mod gich_lr3;
