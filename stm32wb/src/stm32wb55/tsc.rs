#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - interrupt enable register"]
    pub ier: IER,
    #[doc = "0x08 - interrupt clear register"]
    pub icr: ICR,
    #[doc = "0x0c - interrupt status register"]
    pub isr: ISR,
    #[doc = "0x10 - I/O hysteresis control register"]
    pub iohcr: IOHCR,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - I/O analog switch control register"]
    pub ioascr: IOASCR,
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - I/O sampling control register"]
    pub ioscr: IOSCR,
    _reserved7: [u8; 4usize],
    #[doc = "0x28 - I/O channel control register"]
    pub ioccr: IOCCR,
    _reserved8: [u8; 4usize],
    #[doc = "0x30 - I/O group control status register"]
    pub iogcsr: IOGCSR,
    #[doc = "0x34 - I/O group x counter register"]
    pub iog1cr: IOG1CR,
    #[doc = "0x38 - I/O group x counter register"]
    pub iog2cr: IOG2CR,
    #[doc = "0x3c - I/O group x counter register"]
    pub iog3cr: IOG3CR,
    #[doc = "0x40 - I/O group x counter register"]
    pub iog4cr: IOG4CR,
    #[doc = "0x44 - I/O group x counter register"]
    pub iog5cr: IOG5CR,
    #[doc = "0x48 - I/O group x counter register"]
    pub iog6cr: IOG6CR,
    #[doc = "0x4c - I/O group x counter register"]
    pub iog7cr: IOG7CR,
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "control register"]
pub mod cr;
#[doc = "interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`read()` method returns [icr::R](icr::R) reader structure"]
impl crate::Readable for ICR {}
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "interrupt clear register"]
pub mod icr;
#[doc = "interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "`write(|w| ..)` method takes [isr::W](isr::W) writer structure"]
impl crate::Writable for ISR {}
#[doc = "interrupt status register"]
pub mod isr;
#[doc = "I/O hysteresis control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iohcr](iohcr) module"]
pub type IOHCR = crate::Reg<u32, _IOHCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOHCR;
#[doc = "`read()` method returns [iohcr::R](iohcr::R) reader structure"]
impl crate::Readable for IOHCR {}
#[doc = "`write(|w| ..)` method takes [iohcr::W](iohcr::W) writer structure"]
impl crate::Writable for IOHCR {}
#[doc = "I/O hysteresis control register"]
pub mod iohcr;
#[doc = "I/O analog switch control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioascr](ioascr) module"]
pub type IOASCR = crate::Reg<u32, _IOASCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOASCR;
#[doc = "`read()` method returns [ioascr::R](ioascr::R) reader structure"]
impl crate::Readable for IOASCR {}
#[doc = "`write(|w| ..)` method takes [ioascr::W](ioascr::W) writer structure"]
impl crate::Writable for IOASCR {}
#[doc = "I/O analog switch control register"]
pub mod ioascr;
#[doc = "I/O sampling control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioscr](ioscr) module"]
pub type IOSCR = crate::Reg<u32, _IOSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOSCR;
#[doc = "`read()` method returns [ioscr::R](ioscr::R) reader structure"]
impl crate::Readable for IOSCR {}
#[doc = "`write(|w| ..)` method takes [ioscr::W](ioscr::W) writer structure"]
impl crate::Writable for IOSCR {}
#[doc = "I/O sampling control register"]
pub mod ioscr;
#[doc = "I/O channel control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioccr](ioccr) module"]
pub type IOCCR = crate::Reg<u32, _IOCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCCR;
#[doc = "`read()` method returns [ioccr::R](ioccr::R) reader structure"]
impl crate::Readable for IOCCR {}
#[doc = "`write(|w| ..)` method takes [ioccr::W](ioccr::W) writer structure"]
impl crate::Writable for IOCCR {}
#[doc = "I/O channel control register"]
pub mod ioccr;
#[doc = "I/O group control status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iogcsr](iogcsr) module"]
pub type IOGCSR = crate::Reg<u32, _IOGCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOGCSR;
#[doc = "`read()` method returns [iogcsr::R](iogcsr::R) reader structure"]
impl crate::Readable for IOGCSR {}
#[doc = "`write(|w| ..)` method takes [iogcsr::W](iogcsr::W) writer structure"]
impl crate::Writable for IOGCSR {}
#[doc = "I/O group control status register"]
pub mod iogcsr;
#[doc = "I/O group x counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iog1cr](iog1cr) module"]
pub type IOG1CR = crate::Reg<u32, _IOG1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOG1CR;
#[doc = "`read()` method returns [iog1cr::R](iog1cr::R) reader structure"]
impl crate::Readable for IOG1CR {}
#[doc = "I/O group x counter register"]
pub mod iog1cr;
#[doc = "I/O group x counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iog2cr](iog2cr) module"]
pub type IOG2CR = crate::Reg<u32, _IOG2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOG2CR;
#[doc = "`read()` method returns [iog2cr::R](iog2cr::R) reader structure"]
impl crate::Readable for IOG2CR {}
#[doc = "I/O group x counter register"]
pub mod iog2cr;
#[doc = "I/O group x counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iog3cr](iog3cr) module"]
pub type IOG3CR = crate::Reg<u32, _IOG3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOG3CR;
#[doc = "`read()` method returns [iog3cr::R](iog3cr::R) reader structure"]
impl crate::Readable for IOG3CR {}
#[doc = "I/O group x counter register"]
pub mod iog3cr;
#[doc = "I/O group x counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iog4cr](iog4cr) module"]
pub type IOG4CR = crate::Reg<u32, _IOG4CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOG4CR;
#[doc = "`read()` method returns [iog4cr::R](iog4cr::R) reader structure"]
impl crate::Readable for IOG4CR {}
#[doc = "I/O group x counter register"]
pub mod iog4cr;
#[doc = "I/O group x counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iog5cr](iog5cr) module"]
pub type IOG5CR = crate::Reg<u32, _IOG5CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOG5CR;
#[doc = "`read()` method returns [iog5cr::R](iog5cr::R) reader structure"]
impl crate::Readable for IOG5CR {}
#[doc = "I/O group x counter register"]
pub mod iog5cr;
#[doc = "I/O group x counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iog6cr](iog6cr) module"]
pub type IOG6CR = crate::Reg<u32, _IOG6CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOG6CR;
#[doc = "`read()` method returns [iog6cr::R](iog6cr::R) reader structure"]
impl crate::Readable for IOG6CR {}
#[doc = "I/O group x counter register"]
pub mod iog6cr;
#[doc = "I/O group x counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iog7cr](iog7cr) module"]
pub type IOG7CR = crate::Reg<u32, _IOG7CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOG7CR;
#[doc = "`read()` method returns [iog7cr::R](iog7cr::R) reader structure"]
impl crate::Readable for IOG7CR {}
#[doc = "I/O group x counter register"]
pub mod iog7cr;
