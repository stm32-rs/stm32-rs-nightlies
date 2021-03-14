#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN_MCR"]
    pub mcr: MCR,
    #[doc = "0x04 - CAN_MSR"]
    pub msr: MSR,
    #[doc = "0x08 - CAN_TSR"]
    pub tsr: TSR,
    #[doc = "0x0c - CAN_RF0R"]
    pub rfr: [RFR; 2],
    #[doc = "0x14 - CAN_IER"]
    pub ier: IER,
    #[doc = "0x18 - CAN_ESR"]
    pub esr: ESR,
    #[doc = "0x1c - CAN_BTR"]
    pub btr: BTR,
    _reserved7: [u8; 352usize],
    #[doc = "0x180 - CAN Transmit cluster"]
    pub tx: [TX; 3],
    #[doc = "0x1b0 - CAN Receive cluster"]
    pub rx: [RX; 2],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct TX {
    #[doc = "0x00 - CAN_TI0R"]
    pub tir: self::tx::TIR,
    #[doc = "0x04 - CAN_TDT0R"]
    pub tdtr: self::tx::TDTR,
    #[doc = "0x08 - CAN_TDL0R"]
    pub tdlr: self::tx::TDLR,
    #[doc = "0x0c - CAN_TDH0R"]
    pub tdhr: self::tx::TDHR,
}
#[doc = r"Register block"]
#[doc = "CAN Transmit cluster"]
pub mod tx;
#[doc = r"Register block"]
#[repr(C)]
pub struct RX {
    #[doc = "0x00 - CAN_RI0R"]
    pub rir: self::rx::RIR,
    #[doc = "0x04 - CAN_RDT0R"]
    pub rdtr: self::rx::RDTR,
    #[doc = "0x08 - CAN_RDL0R"]
    pub rdlr: self::rx::RDLR,
    #[doc = "0x0c - CAN_RDH0R"]
    pub rdhr: self::rx::RDHR,
}
#[doc = r"Register block"]
#[doc = "CAN Receive cluster"]
pub mod rx;
#[doc = "CAN_MCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "CAN_MCR"]
pub mod mcr;
#[doc = "CAN_MSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](msr) module"]
pub type MSR = crate::Reg<u32, _MSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSR;
#[doc = "`read()` method returns [msr::R](msr::R) reader structure"]
impl crate::Readable for MSR {}
#[doc = "`write(|w| ..)` method takes [msr::W](msr::W) writer structure"]
impl crate::Writable for MSR {}
#[doc = "CAN_MSR"]
pub mod msr;
#[doc = "CAN_TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsr](tsr) module"]
pub type TSR = crate::Reg<u32, _TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSR;
#[doc = "`read()` method returns [tsr::R](tsr::R) reader structure"]
impl crate::Readable for TSR {}
#[doc = "`write(|w| ..)` method takes [tsr::W](tsr::W) writer structure"]
impl crate::Writable for TSR {}
#[doc = "CAN_TSR"]
pub mod tsr;
#[doc = "CAN_RF0R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfr](rfr) module"]
pub type RFR = crate::Reg<u32, _RFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFR;
#[doc = "`read()` method returns [rfr::R](rfr::R) reader structure"]
impl crate::Readable for RFR {}
#[doc = "`write(|w| ..)` method takes [rfr::W](rfr::W) writer structure"]
impl crate::Writable for RFR {}
#[doc = "CAN_RF0R"]
pub mod rfr;
#[doc = "CAN_IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "CAN_IER"]
pub mod ier;
#[doc = "CAN_ESR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esr](esr) module"]
pub type ESR = crate::Reg<u32, _ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESR;
#[doc = "`read()` method returns [esr::R](esr::R) reader structure"]
impl crate::Readable for ESR {}
#[doc = "`write(|w| ..)` method takes [esr::W](esr::W) writer structure"]
impl crate::Writable for ESR {}
#[doc = "CAN_ESR"]
pub mod esr;
#[doc = "CAN_BTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btr](btr) module"]
pub type BTR = crate::Reg<u32, _BTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTR;
#[doc = "`read()` method returns [btr::R](btr::R) reader structure"]
impl crate::Readable for BTR {}
#[doc = "`write(|w| ..)` method takes [btr::W](btr::W) writer structure"]
impl crate::Writable for BTR {}
#[doc = "CAN_BTR"]
pub mod btr;
