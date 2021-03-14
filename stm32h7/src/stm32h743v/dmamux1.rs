#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMAMux - DMA request line multiplexer channel x control register"]
    pub ccr: [CCR; 16],
    _reserved1: [u8; 64usize],
    #[doc = "0x80 - DMAMUX request line multiplexer interrupt channel status register"]
    pub csr: CSR,
    #[doc = "0x84 - DMAMUX request line multiplexer interrupt clear flag register"]
    pub cfr: CFR,
    _reserved3: [u8; 120usize],
    #[doc = "0x100 - DMAMux - DMA request generator channel x control register"]
    pub rgcr: [RGCR; 8],
    _reserved4: [u8; 32usize],
    #[doc = "0x140 - DMAMux - DMA request generator status register"]
    pub rgsr: RGSR,
    #[doc = "0x144 - DMAMux - DMA request generator clear flag register"]
    pub rgcfr: RGCFR,
}
#[doc = "DMAMux - DMA request line multiplexer channel x control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure"]
impl crate::Writable for CCR {}
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod ccr;
#[doc = "DMAMux - DMA request generator channel x control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgcr](rgcr) module"]
pub type RGCR = crate::Reg<u32, _RGCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RGCR;
#[doc = "`read()` method returns [rgcr::R](rgcr::R) reader structure"]
impl crate::Readable for RGCR {}
#[doc = "`write(|w| ..)` method takes [rgcr::W](rgcr::W) writer structure"]
impl crate::Writable for RGCR {}
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod rgcr;
#[doc = "DMAMux - DMA request generator status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgsr](rgsr) module"]
pub type RGSR = crate::Reg<u32, _RGSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RGSR;
#[doc = "`read()` method returns [rgsr::R](rgsr::R) reader structure"]
impl crate::Readable for RGSR {}
#[doc = "DMAMux - DMA request generator status register"]
pub mod rgsr;
#[doc = "DMAMux - DMA request generator clear flag register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgcfr](rgcfr) module"]
pub type RGCFR = crate::Reg<u32, _RGCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RGCFR;
#[doc = "`write(|w| ..)` method takes [rgcfr::W](rgcfr::W) writer structure"]
impl crate::Writable for RGCFR {}
#[doc = "DMAMux - DMA request generator clear flag register"]
pub mod rgcfr;
#[doc = "DMAMUX request line multiplexer interrupt channel status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "DMAMUX request line multiplexer interrupt channel status register"]
pub mod csr;
#[doc = "DMAMUX request line multiplexer interrupt clear flag register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfr](cfr) module"]
pub type CFR = crate::Reg<u32, _CFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFR;
#[doc = "`write(|w| ..)` method takes [cfr::W](cfr::W) writer structure"]
impl crate::Writable for CFR {}
#[doc = "DMAMUX request line multiplexer interrupt clear flag register"]
pub mod cfr;
