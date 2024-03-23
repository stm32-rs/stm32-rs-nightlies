#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ccr: [CCR; 16],
    _reserved1: [u8; 0x40],
    csr: CSR,
    ccfr: CCFR,
    _reserved3: [u8; 0x78],
    rgcr: [RGCR; 4],
    _reserved4: [u8; 0x30],
    rgsr: RGSR,
    rgcfr: RGCFR,
}
impl RegisterBlock {
    #[doc = "0x00..0x40 - DMA Multiplexer Channel %s Control register"]
    #[inline(always)]
    pub const fn ccr(&self, n: usize) -> &CCR {
        &self.ccr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x40 - DMA Multiplexer Channel %s Control register"]
    #[inline(always)]
    pub fn ccr_iter(&self) -> impl Iterator<Item = &CCR> {
        self.ccr.iter()
    }
    #[doc = "0x80 - DMA Multiplexer Channel Status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x84 - DMA Channel Clear Flag Register"]
    #[inline(always)]
    pub const fn ccfr(&self) -> &CCFR {
        &self.ccfr
    }
    #[doc = "0x100..0x110 - DMA Request Generator %s Control Register"]
    #[inline(always)]
    pub const fn rgcr(&self, n: usize) -> &RGCR {
        &self.rgcr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x110 - DMA Request Generator %s Control Register"]
    #[inline(always)]
    pub fn rgcr_iter(&self) -> impl Iterator<Item = &RGCR> {
        self.rgcr.iter()
    }
    #[doc = "0x140 - DMA Request Generator Status Register"]
    #[inline(always)]
    pub const fn rgsr(&self) -> &RGSR {
        &self.rgsr
    }
    #[doc = "0x144 - DMA Request Generator Clear Flag Register"]
    #[inline(always)]
    pub const fn rgcfr(&self) -> &RGCFR {
        &self.rgcfr
    }
}
#[doc = "CCR (rw) register accessor: DMA Multiplexer Channel %s Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCRrs>;
#[doc = "DMA Multiplexer Channel %s Control register"]
pub mod ccr;
#[doc = "CSR (rw) register accessor: DMA Multiplexer Channel Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSRrs>;
#[doc = "DMA Multiplexer Channel Status register"]
pub mod csr;
#[doc = "CCFR (rw) register accessor: DMA Channel Clear Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfr`]
module"]
pub type CCFR = crate::Reg<ccfr::CCFRrs>;
#[doc = "DMA Channel Clear Flag Register"]
pub mod ccfr;
#[doc = "RGCR (rw) register accessor: DMA Request Generator %s Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgcr`]
module"]
pub type RGCR = crate::Reg<rgcr::RGCRrs>;
#[doc = "DMA Request Generator %s Control Register"]
pub mod rgcr;
#[doc = "RGSR (r) register accessor: DMA Request Generator Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgsr`]
module"]
pub type RGSR = crate::Reg<rgsr::RGSRrs>;
#[doc = "DMA Request Generator Status Register"]
pub mod rgsr;
#[doc = "RGCFR (rw) register accessor: DMA Request Generator Clear Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgcfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgcfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgcfr`]
module"]
pub type RGCFR = crate::Reg<rgcfr::RGCFRrs>;
#[doc = "DMA Request Generator Clear Flag Register"]
pub mod rgcfr;
