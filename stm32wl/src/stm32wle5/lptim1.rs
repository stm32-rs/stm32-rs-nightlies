#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - interrupt and status register"]
    pub isr: ISR,
    #[doc = "0x04 - interrupt clear register"]
    pub icr: ICR,
    #[doc = "0x08 - interrupt enable register"]
    pub ier: IER,
    #[doc = "0x0c - configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x10 - control register"]
    pub cr: CR,
    #[doc = "0x14 - compare register"]
    pub cmp: CMP,
    #[doc = "0x18 - autoreload register"]
    pub arr: ARR,
    #[doc = "0x1c - counter register"]
    pub cnt: CNT,
    #[doc = "0x20 - option register"]
    pub lptim1_or: LPTIM1_OR,
    _reserved9: [u8; 4usize],
    #[doc = "0x28 - repetition register"]
    pub rcr: RCR,
}
#[doc = "interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "interrupt and status register"]
pub mod isr;
#[doc = "interrupt clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "interrupt clear register"]
pub mod icr;
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
#[doc = "configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](cfgr) module"]
pub type CFGR = crate::Reg<u32, _CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR;
#[doc = "`read()` method returns [cfgr::R](cfgr::R) reader structure"]
impl crate::Readable for CFGR {}
#[doc = "`write(|w| ..)` method takes [cfgr::W](cfgr::W) writer structure"]
impl crate::Writable for CFGR {}
#[doc = "configuration register"]
pub mod cfgr;
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
#[doc = "compare register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp](cmp) module"]
pub type CMP = crate::Reg<u32, _CMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP;
#[doc = "`read()` method returns [cmp::R](cmp::R) reader structure"]
impl crate::Readable for CMP {}
#[doc = "`write(|w| ..)` method takes [cmp::W](cmp::W) writer structure"]
impl crate::Writable for CMP {}
#[doc = "compare register"]
pub mod cmp;
#[doc = "autoreload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arr](arr) module"]
pub type ARR = crate::Reg<u32, _ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARR;
#[doc = "`read()` method returns [arr::R](arr::R) reader structure"]
impl crate::Readable for ARR {}
#[doc = "`write(|w| ..)` method takes [arr::W](arr::W) writer structure"]
impl crate::Writable for ARR {}
#[doc = "autoreload register"]
pub mod arr;
#[doc = "counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "counter register"]
pub mod cnt;
#[doc = "option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim1_or](lptim1_or) module"]
pub type LPTIM1_OR = crate::Reg<u32, _LPTIM1_OR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM1_OR;
#[doc = "`read()` method returns [lptim1_or::R](lptim1_or::R) reader structure"]
impl crate::Readable for LPTIM1_OR {}
#[doc = "`write(|w| ..)` method takes [lptim1_or::W](lptim1_or::W) writer structure"]
impl crate::Writable for LPTIM1_OR {}
#[doc = "option register"]
pub mod lptim1_or;
#[doc = "repetition register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr](rcr) module"]
pub type RCR = crate::Reg<u32, _RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCR;
#[doc = "`read()` method returns [rcr::R](rcr::R) reader structure"]
impl crate::Readable for RCR {}
#[doc = "`write(|w| ..)` method takes [rcr::W](rcr::W) writer structure"]
impl crate::Writable for RCR {}
#[doc = "repetition register"]
pub mod rcr;
