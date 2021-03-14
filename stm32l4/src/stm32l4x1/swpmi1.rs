#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SWPMI Configuration/Control register"]
    pub cr: CR,
    #[doc = "0x04 - SWPMI Bitrate register"]
    pub brr: BRR,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - SWPMI Interrupt and Status register"]
    pub isr: ISR,
    #[doc = "0x10 - SWPMI Interrupt Flag Clear register"]
    pub icr: ICR,
    #[doc = "0x14 - SWPMI Interrupt Enable register"]
    pub ier: IER,
    #[doc = "0x18 - SWPMI Receive Frame Length register"]
    pub rfl: RFL,
    #[doc = "0x1c - SWPMI Transmit data register"]
    pub tdr: TDR,
    #[doc = "0x20 - SWPMI Receive data register"]
    pub rdr: RDR,
}
#[doc = "SWPMI Configuration/Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "SWPMI Configuration/Control register"]
pub mod cr;
#[doc = "SWPMI Bitrate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brr](brr) module"]
pub type BRR = crate::Reg<u32, _BRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRR;
#[doc = "`read()` method returns [brr::R](brr::R) reader structure"]
impl crate::Readable for BRR {}
#[doc = "`write(|w| ..)` method takes [brr::W](brr::W) writer structure"]
impl crate::Writable for BRR {}
#[doc = "SWPMI Bitrate register"]
pub mod brr;
#[doc = "SWPMI Interrupt and Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "SWPMI Interrupt and Status register"]
pub mod isr;
#[doc = "SWPMI Interrupt Flag Clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "SWPMI Interrupt Flag Clear register"]
pub mod icr;
#[doc = "SWPMI Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "SWPMI Interrupt Enable register"]
pub mod ier;
#[doc = "SWPMI Receive Frame Length register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfl](rfl) module"]
pub type RFL = crate::Reg<u32, _RFL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFL;
#[doc = "`read()` method returns [rfl::R](rfl::R) reader structure"]
impl crate::Readable for RFL {}
#[doc = "SWPMI Receive Frame Length register"]
pub mod rfl;
#[doc = "SWPMI Transmit data register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdr](tdr) module"]
pub type TDR = crate::Reg<u32, _TDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDR;
#[doc = "`write(|w| ..)` method takes [tdr::W](tdr::W) writer structure"]
impl crate::Writable for TDR {}
#[doc = "SWPMI Transmit data register"]
pub mod tdr;
#[doc = "SWPMI Receive data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdr](rdr) module"]
pub type RDR = crate::Reg<u32, _RDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDR;
#[doc = "`read()` method returns [rdr::R](rdr::R) reader structure"]
impl crate::Readable for RDR {}
#[doc = "SWPMI Receive data register"]
pub mod rdr;
