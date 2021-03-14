#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - peripheral mode configuration register"]
    pub pmcr: PMCR,
    #[doc = "0x08 - external interrupt configuration register 1"]
    pub exticr1: EXTICR1,
    #[doc = "0x0c - external interrupt configuration register 2"]
    pub exticr2: EXTICR2,
    #[doc = "0x10 - external interrupt configuration register 3"]
    pub exticr3: EXTICR3,
    #[doc = "0x14 - external interrupt configuration register 4"]
    pub exticr4: EXTICR4,
    _reserved5: [u8; 8usize],
    #[doc = "0x20 - compensation cell control/status register"]
    pub cccsr: CCCSR,
    #[doc = "0x24 - SYSCFG compensation cell value register"]
    pub ccvr: CCVR,
    #[doc = "0x28 - SYSCFG compensation cell code register"]
    pub cccr: CCCR,
    _reserved8: [u8; 236usize],
    #[doc = "0x118 - SYSCFG timer break lockup register"]
    pub syscfg_brk_lockupr: SYSCFG_BRK_LOCKUPR,
}
#[doc = "peripheral mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmcr](pmcr) module"]
pub type PMCR = crate::Reg<u32, _PMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMCR;
#[doc = "`read()` method returns [pmcr::R](pmcr::R) reader structure"]
impl crate::Readable for PMCR {}
#[doc = "`write(|w| ..)` method takes [pmcr::W](pmcr::W) writer structure"]
impl crate::Writable for PMCR {}
#[doc = "peripheral mode configuration register"]
pub mod pmcr;
#[doc = "external interrupt configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr1](exticr1) module"]
pub type EXTICR1 = crate::Reg<u32, _EXTICR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR1;
#[doc = "`read()` method returns [exticr1::R](exticr1::R) reader structure"]
impl crate::Readable for EXTICR1 {}
#[doc = "`write(|w| ..)` method takes [exticr1::W](exticr1::W) writer structure"]
impl crate::Writable for EXTICR1 {}
#[doc = "external interrupt configuration register 1"]
pub mod exticr1;
#[doc = "external interrupt configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr2](exticr2) module"]
pub type EXTICR2 = crate::Reg<u32, _EXTICR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR2;
#[doc = "`read()` method returns [exticr2::R](exticr2::R) reader structure"]
impl crate::Readable for EXTICR2 {}
#[doc = "`write(|w| ..)` method takes [exticr2::W](exticr2::W) writer structure"]
impl crate::Writable for EXTICR2 {}
#[doc = "external interrupt configuration register 2"]
pub mod exticr2;
#[doc = "external interrupt configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr3](exticr3) module"]
pub type EXTICR3 = crate::Reg<u32, _EXTICR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR3;
#[doc = "`read()` method returns [exticr3::R](exticr3::R) reader structure"]
impl crate::Readable for EXTICR3 {}
#[doc = "`write(|w| ..)` method takes [exticr3::W](exticr3::W) writer structure"]
impl crate::Writable for EXTICR3 {}
#[doc = "external interrupt configuration register 3"]
pub mod exticr3;
#[doc = "external interrupt configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr4](exticr4) module"]
pub type EXTICR4 = crate::Reg<u32, _EXTICR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR4;
#[doc = "`read()` method returns [exticr4::R](exticr4::R) reader structure"]
impl crate::Readable for EXTICR4 {}
#[doc = "`write(|w| ..)` method takes [exticr4::W](exticr4::W) writer structure"]
impl crate::Writable for EXTICR4 {}
#[doc = "external interrupt configuration register 4"]
pub mod exticr4;
#[doc = "compensation cell control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cccsr](cccsr) module"]
pub type CCCSR = crate::Reg<u32, _CCCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCCSR;
#[doc = "`read()` method returns [cccsr::R](cccsr::R) reader structure"]
impl crate::Readable for CCCSR {}
#[doc = "`write(|w| ..)` method takes [cccsr::W](cccsr::W) writer structure"]
impl crate::Writable for CCCSR {}
#[doc = "compensation cell control/status register"]
pub mod cccsr;
#[doc = "SYSCFG compensation cell value register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccvr](ccvr) module"]
pub type CCVR = crate::Reg<u32, _CCVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCVR;
#[doc = "`read()` method returns [ccvr::R](ccvr::R) reader structure"]
impl crate::Readable for CCVR {}
#[doc = "SYSCFG compensation cell value register"]
pub mod ccvr;
#[doc = "SYSCFG compensation cell code register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cccr](cccr) module"]
pub type CCCR = crate::Reg<u32, _CCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCCR;
#[doc = "`read()` method returns [cccr::R](cccr::R) reader structure"]
impl crate::Readable for CCCR {}
#[doc = "`write(|w| ..)` method takes [cccr::W](cccr::W) writer structure"]
impl crate::Writable for CCCR {}
#[doc = "SYSCFG compensation cell code register"]
pub mod cccr;
#[doc = "SYSCFG timer break lockup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_brk_lockupr](syscfg_brk_lockupr) module"]
pub type SYSCFG_BRK_LOCKUPR = crate::Reg<u32, _SYSCFG_BRK_LOCKUPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_BRK_LOCKUPR;
#[doc = "`read()` method returns [syscfg_brk_lockupr::R](syscfg_brk_lockupr::R) reader structure"]
impl crate::Readable for SYSCFG_BRK_LOCKUPR {}
#[doc = "`write(|w| ..)` method takes [syscfg_brk_lockupr::W](syscfg_brk_lockupr::W) writer structure"]
impl crate::Writable for SYSCFG_BRK_LOCKUPR {}
#[doc = "SYSCFG timer break lockup register"]
pub mod syscfg_brk_lockupr;
