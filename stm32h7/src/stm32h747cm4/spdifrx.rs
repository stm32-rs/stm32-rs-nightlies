#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    imr: IMR,
    sr: SR,
    ifcr: IFCR,
    _reserved_4_dr_: [u8; 0x04],
    csr: CSR,
    dir: DIR,
    _reserved7: [u8; 0x03d8],
    verr: VERR,
    idr: IDR,
    sidr: SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - Interrupt mask register"]
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    #[doc = "0x08 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x0c - Interrupt Flag Clear register"]
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    #[doc = "0x10 - Data input register"]
    #[inline(always)]
    pub const fn dr_10(&self) -> &DR_10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Data input register"]
    #[inline(always)]
    pub const fn dr_01(&self) -> &DR_01 {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Data input register"]
    #[inline(always)]
    pub const fn dr_00(&self) -> &DR_00 {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x14 - Channel Status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x18 - Debug Information register"]
    #[inline(always)]
    pub const fn dir(&self) -> &DIR {
        &self.dir
    }
    #[doc = "0x3f4 - SPDIFRX version register"]
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    #[doc = "0x3f8 - SPDIFRX identification register"]
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    #[doc = "0x3fc - SPDIFRX size identification register"]
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
#[doc = "CR (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "Control register"]
pub mod cr;
#[doc = "IMR (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
pub type IMR = crate::Reg<imr::IMRrs>;
#[doc = "Interrupt mask register"]
pub mod imr;
#[doc = "SR (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "Status register"]
pub mod sr;
#[doc = "IFCR (w) register accessor: Interrupt Flag Clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`]
module"]
pub type IFCR = crate::Reg<ifcr::IFCRrs>;
#[doc = "Interrupt Flag Clear register"]
pub mod ifcr;
#[doc = "DR_00 (r) register accessor: Data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr_00::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr_00`]
module"]
pub type DR_00 = crate::Reg<dr_00::DR_00rs>;
#[doc = "Data input register"]
pub mod dr_00;
#[doc = "CSR (r) register accessor: Channel Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSRrs>;
#[doc = "Channel Status register"]
pub mod csr;
#[doc = "DIR (r) register accessor: Debug Information register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dir::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dir`]
module"]
pub type DIR = crate::Reg<dir::DIRrs>;
#[doc = "Debug Information register"]
pub mod dir;
#[doc = "VERR (r) register accessor: SPDIFRX version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verr`]
module"]
pub type VERR = crate::Reg<verr::VERRrs>;
#[doc = "SPDIFRX version register"]
pub mod verr;
#[doc = "IDR (r) register accessor: SPDIFRX identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
pub type IDR = crate::Reg<idr::IDRrs>;
#[doc = "SPDIFRX identification register"]
pub mod idr;
#[doc = "SIDR (r) register accessor: SPDIFRX size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidr`]
module"]
pub type SIDR = crate::Reg<sidr::SIDRrs>;
#[doc = "SPDIFRX size identification register"]
pub mod sidr;
#[doc = "DR_01 (r) register accessor: Data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr_01::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr_01`]
module"]
pub type DR_01 = crate::Reg<dr_01::DR_01rs>;
#[doc = "Data input register"]
pub mod dr_01;
#[doc = "DR_10 (r) register accessor: Data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr_10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr_10`]
module"]
pub type DR_10 = crate::Reg<dr_10::DR_10rs>;
#[doc = "Data input register"]
pub mod dr_10;
