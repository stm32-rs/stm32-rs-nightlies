#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - Control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - Control register 3"]
    pub cr3: CR3,
    #[doc = "0x0c - Baud rate register"]
    pub brr: BRR,
    #[doc = "0x10 - Guard time and prescaler register"]
    pub gtpr: GTPR,
    #[doc = "0x14 - Receiver timeout register"]
    pub rtor: RTOR,
    #[doc = "0x18 - Request register"]
    pub rqr: RQR,
    #[doc = "0x1c - Interrupt & status register"]
    pub isr: ISR,
    #[doc = "0x20 - Interrupt flag clear register"]
    pub icr: ICR,
    #[doc = "0x24 - Receive data register"]
    pub rdr: RDR,
    #[doc = "0x28 - Transmit data register"]
    pub tdr: TDR,
    #[doc = "0x2c - USART prescaler register"]
    pub presc: PRESC,
}
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](cr1) module"]
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
#[doc = "`read()` method returns [cr1::R](cr1::R) reader structure"]
impl crate::Readable for CR1 {}
#[doc = "`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure"]
impl crate::Writable for CR1 {}
#[doc = "Control register 1"]
pub mod cr1;
#[doc = "Control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](cr2) module"]
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
#[doc = "`read()` method returns [cr2::R](cr2::R) reader structure"]
impl crate::Readable for CR2 {}
#[doc = "`write(|w| ..)` method takes [cr2::W](cr2::W) writer structure"]
impl crate::Writable for CR2 {}
#[doc = "Control register 2"]
pub mod cr2;
#[doc = "Control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr3](cr3) module"]
pub type CR3 = crate::Reg<u32, _CR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR3;
#[doc = "`read()` method returns [cr3::R](cr3::R) reader structure"]
impl crate::Readable for CR3 {}
#[doc = "`write(|w| ..)` method takes [cr3::W](cr3::W) writer structure"]
impl crate::Writable for CR3 {}
#[doc = "Control register 3"]
pub mod cr3;
#[doc = "Baud rate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brr](brr) module"]
pub type BRR = crate::Reg<u32, _BRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRR;
#[doc = "`read()` method returns [brr::R](brr::R) reader structure"]
impl crate::Readable for BRR {}
#[doc = "`write(|w| ..)` method takes [brr::W](brr::W) writer structure"]
impl crate::Writable for BRR {}
#[doc = "Baud rate register"]
pub mod brr;
#[doc = "Guard time and prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtpr](gtpr) module"]
pub type GTPR = crate::Reg<u32, _GTPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GTPR;
#[doc = "`read()` method returns [gtpr::R](gtpr::R) reader structure"]
impl crate::Readable for GTPR {}
#[doc = "`write(|w| ..)` method takes [gtpr::W](gtpr::W) writer structure"]
impl crate::Writable for GTPR {}
#[doc = "Guard time and prescaler register"]
pub mod gtpr;
#[doc = "Receiver timeout register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtor](rtor) module"]
pub type RTOR = crate::Reg<u32, _RTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTOR;
#[doc = "`read()` method returns [rtor::R](rtor::R) reader structure"]
impl crate::Readable for RTOR {}
#[doc = "`write(|w| ..)` method takes [rtor::W](rtor::W) writer structure"]
impl crate::Writable for RTOR {}
#[doc = "Receiver timeout register"]
pub mod rtor;
#[doc = "Request register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rqr](rqr) module"]
pub type RQR = crate::Reg<u32, _RQR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RQR;
#[doc = "`write(|w| ..)` method takes [rqr::W](rqr::W) writer structure"]
impl crate::Writable for RQR {}
#[doc = "Request register"]
pub mod rqr;
#[doc = "Interrupt & status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "Interrupt & status register"]
pub mod isr;
#[doc = "Interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "Interrupt flag clear register"]
pub mod icr;
#[doc = "Receive data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdr](rdr) module"]
pub type RDR = crate::Reg<u32, _RDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDR;
#[doc = "`read()` method returns [rdr::R](rdr::R) reader structure"]
impl crate::Readable for RDR {}
#[doc = "Receive data register"]
pub mod rdr;
#[doc = "Transmit data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdr](tdr) module"]
pub type TDR = crate::Reg<u32, _TDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDR;
#[doc = "`read()` method returns [tdr::R](tdr::R) reader structure"]
impl crate::Readable for TDR {}
#[doc = "`write(|w| ..)` method takes [tdr::W](tdr::W) writer structure"]
impl crate::Writable for TDR {}
#[doc = "Transmit data register"]
pub mod tdr;
#[doc = "USART prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presc](presc) module"]
pub type PRESC = crate::Reg<u32, _PRESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESC;
#[doc = "`read()` method returns [presc::R](presc::R) reader structure"]
impl crate::Readable for PRESC {}
#[doc = "`write(|w| ..)` method takes [presc::W](presc::W) writer structure"]
impl crate::Writable for PRESC {}
#[doc = "USART prescaler register"]
pub mod presc;
