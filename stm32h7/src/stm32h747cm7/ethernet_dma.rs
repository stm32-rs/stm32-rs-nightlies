#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA mode register"]
    pub dmamr: DMAMR,
    #[doc = "0x04 - System bus mode register"]
    pub dmasbmr: DMASBMR,
    #[doc = "0x08 - Interrupt status register"]
    pub dmaisr: DMAISR,
    #[doc = "0x0c - Debug status register"]
    pub dmadsr: DMADSR,
    _reserved4: [u8; 240usize],
    #[doc = "0x100 - Channel control register"]
    pub dmaccr: DMACCR,
    #[doc = "0x104 - Channel transmit control register"]
    pub dmactx_cr: DMACTXCR,
    #[doc = "0x108 - Channel receive control register"]
    pub dmacrx_cr: DMACRXCR,
    _reserved7: [u8; 8usize],
    #[doc = "0x114 - Channel Tx descriptor list address register"]
    pub dmactx_dlar: DMACTXDLAR,
    _reserved8: [u8; 4usize],
    #[doc = "0x11c - Channel Rx descriptor list address register"]
    pub dmacrx_dlar: DMACRXDLAR,
    #[doc = "0x120 - Channel Tx descriptor tail pointer register"]
    pub dmactx_dtpr: DMACTXDTPR,
    _reserved10: [u8; 4usize],
    #[doc = "0x128 - Channel Rx descriptor tail pointer register"]
    pub dmacrx_dtpr: DMACRXDTPR,
    #[doc = "0x12c - Channel Tx descriptor ring length register"]
    pub dmactx_rlr: DMACTXRLR,
    #[doc = "0x130 - Channel Rx descriptor ring length register"]
    pub dmacrx_rlr: DMACRXRLR,
    #[doc = "0x134 - Channel interrupt enable register"]
    pub dmacier: DMACIER,
    #[doc = "0x138 - Channel Rx interrupt watchdog timer register"]
    pub dmacrx_iwtr: DMACRXIWTR,
    _reserved15: [u8; 8usize],
    #[doc = "0x144 - Channel current application transmit descriptor register"]
    pub dmaccatx_dr: DMACCATXDR,
    _reserved16: [u8; 4usize],
    #[doc = "0x14c - Channel current application receive descriptor register"]
    pub dmaccarx_dr: DMACCARXDR,
    _reserved17: [u8; 4usize],
    #[doc = "0x154 - Channel current application transmit buffer register"]
    pub dmaccatx_br: DMACCATXBR,
    _reserved18: [u8; 4usize],
    #[doc = "0x15c - Channel current application receive buffer register"]
    pub dmaccarx_br: DMACCARXBR,
    #[doc = "0x160 - Channel status register"]
    pub dmacsr: DMACSR,
    _reserved20: [u8; 8usize],
    #[doc = "0x16c - Channel missed frame count register"]
    pub dmacmfcr: DMACMFCR,
}
#[doc = "DMA mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamr](dmamr) module"]
pub type DMAMR = crate::Reg<u32, _DMAMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMR;
#[doc = "`read()` method returns [dmamr::R](dmamr::R) reader structure"]
impl crate::Readable for DMAMR {}
#[doc = "`write(|w| ..)` method takes [dmamr::W](dmamr::W) writer structure"]
impl crate::Writable for DMAMR {}
#[doc = "DMA mode register"]
pub mod dmamr;
#[doc = "System bus mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmasbmr](dmasbmr) module"]
pub type DMASBMR = crate::Reg<u32, _DMASBMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMASBMR;
#[doc = "`read()` method returns [dmasbmr::R](dmasbmr::R) reader structure"]
impl crate::Readable for DMASBMR {}
#[doc = "`write(|w| ..)` method takes [dmasbmr::W](dmasbmr::W) writer structure"]
impl crate::Writable for DMASBMR {}
#[doc = "System bus mode register"]
pub mod dmasbmr;
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaisr](dmaisr) module"]
pub type DMAISR = crate::Reg<u32, _DMAISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAISR;
#[doc = "`read()` method returns [dmaisr::R](dmaisr::R) reader structure"]
impl crate::Readable for DMAISR {}
#[doc = "`write(|w| ..)` method takes [dmaisr::W](dmaisr::W) writer structure"]
impl crate::Writable for DMAISR {}
#[doc = "Interrupt status register"]
pub mod dmaisr;
#[doc = "Debug status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmadsr](dmadsr) module"]
pub type DMADSR = crate::Reg<u32, _DMADSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMADSR;
#[doc = "`read()` method returns [dmadsr::R](dmadsr::R) reader structure"]
impl crate::Readable for DMADSR {}
#[doc = "`write(|w| ..)` method takes [dmadsr::W](dmadsr::W) writer structure"]
impl crate::Writable for DMADSR {}
#[doc = "Debug status register"]
pub mod dmadsr;
#[doc = "Channel control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaccr](dmaccr) module"]
pub type DMACCR = crate::Reg<u32, _DMACCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACCR;
#[doc = "`read()` method returns [dmaccr::R](dmaccr::R) reader structure"]
impl crate::Readable for DMACCR {}
#[doc = "`write(|w| ..)` method takes [dmaccr::W](dmaccr::W) writer structure"]
impl crate::Writable for DMACCR {}
#[doc = "Channel control register"]
pub mod dmaccr;
#[doc = "Channel transmit control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactx_cr](dmactx_cr) module"]
pub type DMACTXCR = crate::Reg<u32, _DMACTXCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTXCR;
#[doc = "`read()` method returns [dmactx_cr::R](dmactx_cr::R) reader structure"]
impl crate::Readable for DMACTXCR {}
#[doc = "`write(|w| ..)` method takes [dmactx_cr::W](dmactx_cr::W) writer structure"]
impl crate::Writable for DMACTXCR {}
#[doc = "Channel transmit control register"]
pub mod dmactx_cr;
#[doc = "Channel receive control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacrx_cr](dmacrx_cr) module"]
pub type DMACRXCR = crate::Reg<u32, _DMACRXCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACRXCR;
#[doc = "`read()` method returns [dmacrx_cr::R](dmacrx_cr::R) reader structure"]
impl crate::Readable for DMACRXCR {}
#[doc = "`write(|w| ..)` method takes [dmacrx_cr::W](dmacrx_cr::W) writer structure"]
impl crate::Writable for DMACRXCR {}
#[doc = "Channel receive control register"]
pub mod dmacrx_cr;
#[doc = "Channel Tx descriptor list address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactx_dlar](dmactx_dlar) module"]
pub type DMACTXDLAR = crate::Reg<u32, _DMACTXDLAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTXDLAR;
#[doc = "`read()` method returns [dmactx_dlar::R](dmactx_dlar::R) reader structure"]
impl crate::Readable for DMACTXDLAR {}
#[doc = "`write(|w| ..)` method takes [dmactx_dlar::W](dmactx_dlar::W) writer structure"]
impl crate::Writable for DMACTXDLAR {}
#[doc = "Channel Tx descriptor list address register"]
pub mod dmactx_dlar;
#[doc = "Channel Rx descriptor list address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacrx_dlar](dmacrx_dlar) module"]
pub type DMACRXDLAR = crate::Reg<u32, _DMACRXDLAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACRXDLAR;
#[doc = "`read()` method returns [dmacrx_dlar::R](dmacrx_dlar::R) reader structure"]
impl crate::Readable for DMACRXDLAR {}
#[doc = "`write(|w| ..)` method takes [dmacrx_dlar::W](dmacrx_dlar::W) writer structure"]
impl crate::Writable for DMACRXDLAR {}
#[doc = "Channel Rx descriptor list address register"]
pub mod dmacrx_dlar;
#[doc = "Channel Tx descriptor tail pointer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactx_dtpr](dmactx_dtpr) module"]
pub type DMACTXDTPR = crate::Reg<u32, _DMACTXDTPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTXDTPR;
#[doc = "`read()` method returns [dmactx_dtpr::R](dmactx_dtpr::R) reader structure"]
impl crate::Readable for DMACTXDTPR {}
#[doc = "`write(|w| ..)` method takes [dmactx_dtpr::W](dmactx_dtpr::W) writer structure"]
impl crate::Writable for DMACTXDTPR {}
#[doc = "Channel Tx descriptor tail pointer register"]
pub mod dmactx_dtpr;
#[doc = "Channel Rx descriptor tail pointer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacrx_dtpr](dmacrx_dtpr) module"]
pub type DMACRXDTPR = crate::Reg<u32, _DMACRXDTPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACRXDTPR;
#[doc = "`read()` method returns [dmacrx_dtpr::R](dmacrx_dtpr::R) reader structure"]
impl crate::Readable for DMACRXDTPR {}
#[doc = "`write(|w| ..)` method takes [dmacrx_dtpr::W](dmacrx_dtpr::W) writer structure"]
impl crate::Writable for DMACRXDTPR {}
#[doc = "Channel Rx descriptor tail pointer register"]
pub mod dmacrx_dtpr;
#[doc = "Channel Tx descriptor ring length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactx_rlr](dmactx_rlr) module"]
pub type DMACTXRLR = crate::Reg<u32, _DMACTXRLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTXRLR;
#[doc = "`read()` method returns [dmactx_rlr::R](dmactx_rlr::R) reader structure"]
impl crate::Readable for DMACTXRLR {}
#[doc = "`write(|w| ..)` method takes [dmactx_rlr::W](dmactx_rlr::W) writer structure"]
impl crate::Writable for DMACTXRLR {}
#[doc = "Channel Tx descriptor ring length register"]
pub mod dmactx_rlr;
#[doc = "Channel Rx descriptor ring length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacrx_rlr](dmacrx_rlr) module"]
pub type DMACRXRLR = crate::Reg<u32, _DMACRXRLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACRXRLR;
#[doc = "`read()` method returns [dmacrx_rlr::R](dmacrx_rlr::R) reader structure"]
impl crate::Readable for DMACRXRLR {}
#[doc = "`write(|w| ..)` method takes [dmacrx_rlr::W](dmacrx_rlr::W) writer structure"]
impl crate::Writable for DMACRXRLR {}
#[doc = "Channel Rx descriptor ring length register"]
pub mod dmacrx_rlr;
#[doc = "Channel interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacier](dmacier) module"]
pub type DMACIER = crate::Reg<u32, _DMACIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACIER;
#[doc = "`read()` method returns [dmacier::R](dmacier::R) reader structure"]
impl crate::Readable for DMACIER {}
#[doc = "`write(|w| ..)` method takes [dmacier::W](dmacier::W) writer structure"]
impl crate::Writable for DMACIER {}
#[doc = "Channel interrupt enable register"]
pub mod dmacier;
#[doc = "Channel Rx interrupt watchdog timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacrx_iwtr](dmacrx_iwtr) module"]
pub type DMACRXIWTR = crate::Reg<u32, _DMACRXIWTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACRXIWTR;
#[doc = "`read()` method returns [dmacrx_iwtr::R](dmacrx_iwtr::R) reader structure"]
impl crate::Readable for DMACRXIWTR {}
#[doc = "`write(|w| ..)` method takes [dmacrx_iwtr::W](dmacrx_iwtr::W) writer structure"]
impl crate::Writable for DMACRXIWTR {}
#[doc = "Channel Rx interrupt watchdog timer register"]
pub mod dmacrx_iwtr;
#[doc = "Channel current application transmit descriptor register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaccatx_dr](dmaccatx_dr) module"]
pub type DMACCATXDR = crate::Reg<u32, _DMACCATXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACCATXDR;
#[doc = "`read()` method returns [dmaccatx_dr::R](dmaccatx_dr::R) reader structure"]
impl crate::Readable for DMACCATXDR {}
#[doc = "`write(|w| ..)` method takes [dmaccatx_dr::W](dmaccatx_dr::W) writer structure"]
impl crate::Writable for DMACCATXDR {}
#[doc = "Channel current application transmit descriptor register"]
pub mod dmaccatx_dr;
#[doc = "Channel current application receive descriptor register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaccarx_dr](dmaccarx_dr) module"]
pub type DMACCARXDR = crate::Reg<u32, _DMACCARXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACCARXDR;
#[doc = "`read()` method returns [dmaccarx_dr::R](dmaccarx_dr::R) reader structure"]
impl crate::Readable for DMACCARXDR {}
#[doc = "`write(|w| ..)` method takes [dmaccarx_dr::W](dmaccarx_dr::W) writer structure"]
impl crate::Writable for DMACCARXDR {}
#[doc = "Channel current application receive descriptor register"]
pub mod dmaccarx_dr;
#[doc = "Channel current application transmit buffer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaccatx_br](dmaccatx_br) module"]
pub type DMACCATXBR = crate::Reg<u32, _DMACCATXBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACCATXBR;
#[doc = "`read()` method returns [dmaccatx_br::R](dmaccatx_br::R) reader structure"]
impl crate::Readable for DMACCATXBR {}
#[doc = "`write(|w| ..)` method takes [dmaccatx_br::W](dmaccatx_br::W) writer structure"]
impl crate::Writable for DMACCATXBR {}
#[doc = "Channel current application transmit buffer register"]
pub mod dmaccatx_br;
#[doc = "Channel current application receive buffer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaccarx_br](dmaccarx_br) module"]
pub type DMACCARXBR = crate::Reg<u32, _DMACCARXBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACCARXBR;
#[doc = "`read()` method returns [dmaccarx_br::R](dmaccarx_br::R) reader structure"]
impl crate::Readable for DMACCARXBR {}
#[doc = "`write(|w| ..)` method takes [dmaccarx_br::W](dmaccarx_br::W) writer structure"]
impl crate::Writable for DMACCARXBR {}
#[doc = "Channel current application receive buffer register"]
pub mod dmaccarx_br;
#[doc = "Channel status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacsr](dmacsr) module"]
pub type DMACSR = crate::Reg<u32, _DMACSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACSR;
#[doc = "`read()` method returns [dmacsr::R](dmacsr::R) reader structure"]
impl crate::Readable for DMACSR {}
#[doc = "`write(|w| ..)` method takes [dmacsr::W](dmacsr::W) writer structure"]
impl crate::Writable for DMACSR {}
#[doc = "Channel status register"]
pub mod dmacsr;
#[doc = "Channel missed frame count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacmfcr](dmacmfcr) module"]
pub type DMACMFCR = crate::Reg<u32, _DMACMFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACMFCR;
#[doc = "`read()` method returns [dmacmfcr::R](dmacmfcr::R) reader structure"]
impl crate::Readable for DMACMFCR {}
#[doc = "`write(|w| ..)` method takes [dmacmfcr::W](dmacmfcr::W) writer structure"]
impl crate::Writable for DMACMFCR {}
#[doc = "Channel missed frame count register"]
pub mod dmacmfcr;
