#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ccr: [CCR; 7],
    _reserved1: [u8; 0xe4],
    rgcr: [RGCR; 4],
    _reserved2: [u8; 0x30],
    rgsr: RGSR,
    rgcfr: RGCFR,
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
