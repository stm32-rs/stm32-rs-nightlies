#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ccr: [CCR; 7],
    _reserved1: [u8; 0x64],
    csr: CSR,
    cfr: CFR,
    _reserved3: [u8; 0x78],
    rgcr: [RGCR; 4],
    _reserved4: [u8; 0x30],
    rgsr: RGSR,
    rgcfr: RGCFR,
    _reserved6: [u8; 0x02a4],
    hwcfgr2: HWCFGR2,
    hwcfgr1: HWCFGR1,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    #[doc = "0x00..0x1c - DMA Multiplexer Channel %s Control register"]
    #[inline(always)]
    pub const fn ccr(&self, n: usize) -> &CCR {
        &self.ccr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x1c - DMA Multiplexer Channel %s Control register"]
    #[inline(always)]
    pub fn ccr_iter(&self) -> impl Iterator<Item = &CCR> {
        self.ccr.iter()
    }
    #[doc = "0x80 - DMAMUX request line multiplexer interrupt channel status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x84 - DMAMUX request line multiplexer interrupt clear flag register"]
    #[inline(always)]
    pub const fn cfr(&self) -> &CFR {
        &self.cfr
    }
    #[doc = "0x100..0x110 - DMAMux - DMA request generator channel x control register"]
    #[inline(always)]
    pub const fn rgcr(&self, n: usize) -> &RGCR {
        &self.rgcr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x110 - DMAMux - DMA request generator channel x control register"]
    #[inline(always)]
    pub fn rgcr_iter(&self) -> impl Iterator<Item = &RGCR> {
        self.rgcr.iter()
    }
    #[doc = "0x140 - DMAMux - DMA request generator status register"]
    #[inline(always)]
    pub const fn rgsr(&self) -> &RGSR {
        &self.rgsr
    }
    #[doc = "0x144 - DMAMux - DMA request generator clear flag register"]
    #[inline(always)]
    pub const fn rgcfr(&self) -> &RGCFR {
        &self.rgcfr
    }
    #[doc = "0x3ec - DMAMUX hardware configuration 2 register"]
    #[inline(always)]
    pub const fn hwcfgr2(&self) -> &HWCFGR2 {
        &self.hwcfgr2
    }
    #[doc = "0x3f0 - DMAMUX hardware configuration 1 register"]
    #[inline(always)]
    pub const fn hwcfgr1(&self) -> &HWCFGR1 {
        &self.hwcfgr1
    }
    #[doc = "0x3f4 - DMAMUX version register"]
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    #[doc = "0x3f8 - DMAMUX IP identification register"]
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    #[doc = "0x3fc - DMAMUX size identification register"]
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
#[doc = "CCR (rw) register accessor: DMA Multiplexer Channel %s Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCRrs>;
#[doc = "DMA Multiplexer Channel %s Control register"]
pub mod ccr;
#[doc = "RGCR (rw) register accessor: DMAMux - DMA request generator channel x control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgcr`]
module"]
pub type RGCR = crate::Reg<rgcr::RGCRrs>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod rgcr;
#[doc = "RGSR (r) register accessor: DMAMux - DMA request generator status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgsr`]
module"]
pub type RGSR = crate::Reg<rgsr::RGSRrs>;
#[doc = "DMAMux - DMA request generator status register"]
pub mod rgsr;
#[doc = "RGCFR (w) register accessor: DMAMux - DMA request generator clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgcfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgcfr`]
module"]
pub type RGCFR = crate::Reg<rgcfr::RGCFRrs>;
#[doc = "DMAMux - DMA request generator clear flag register"]
pub mod rgcfr;
#[doc = "CSR (r) register accessor: DMAMUX request line multiplexer interrupt channel status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSRrs>;
#[doc = "DMAMUX request line multiplexer interrupt channel status register"]
pub mod csr;
#[doc = "CFR (w) register accessor: DMAMUX request line multiplexer interrupt clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfr`]
module"]
pub type CFR = crate::Reg<cfr::CFRrs>;
#[doc = "DMAMUX request line multiplexer interrupt clear flag register"]
pub mod cfr;
#[doc = "SIDR (r) register accessor: DMAMUX size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidr`]
module"]
pub type SIDR = crate::Reg<sidr::SIDRrs>;
#[doc = "DMAMUX size identification register"]
pub mod sidr;
#[doc = "IPIDR (r) register accessor: DMAMUX IP identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipidr`]
module"]
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
#[doc = "DMAMUX IP identification register"]
pub mod ipidr;
#[doc = "VERR (r) register accessor: DMAMUX version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verr`]
module"]
pub type VERR = crate::Reg<verr::VERRrs>;
#[doc = "DMAMUX version register"]
pub mod verr;
#[doc = "HWCFGR1 (r) register accessor: DMAMUX hardware configuration 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr1`]
module"]
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1rs>;
#[doc = "DMAMUX hardware configuration 1 register"]
pub mod hwcfgr1;
#[doc = "HWCFGR2 (r) register accessor: DMAMUX hardware configuration 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr2`]
module"]
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2rs>;
#[doc = "DMAMUX hardware configuration 2 register"]
pub mod hwcfgr2;
