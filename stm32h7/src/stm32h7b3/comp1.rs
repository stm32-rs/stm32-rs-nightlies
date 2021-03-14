#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator status register"]
    pub comp1_sr: COMP1_SR,
    #[doc = "0x04 - Comparator interrupt clear flag register"]
    pub comp1_icfr: COMP1_ICFR,
    #[doc = "0x08 - Comparator option register"]
    pub comp1_or: COMP1_OR,
    #[doc = "0x0c - Comparator configuration register 1"]
    pub comp1_cfgr1: COMP1_CFGR1,
    #[doc = "0x10 - Comparator configuration register 2"]
    pub comp1_cfgr2: COMP1_CFGR2,
}
#[doc = "Comparator status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1_sr](comp1_sr) module"]
pub type COMP1_SR = crate::Reg<u32, _COMP1_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP1_SR;
#[doc = "`read()` method returns [comp1_sr::R](comp1_sr::R) reader structure"]
impl crate::Readable for COMP1_SR {}
#[doc = "Comparator status register"]
pub mod comp1_sr;
#[doc = "Comparator interrupt clear flag register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1_icfr](comp1_icfr) module"]
pub type COMP1_ICFR = crate::Reg<u32, _COMP1_ICFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP1_ICFR;
#[doc = "`write(|w| ..)` method takes [comp1_icfr::W](comp1_icfr::W) writer structure"]
impl crate::Writable for COMP1_ICFR {}
#[doc = "Comparator interrupt clear flag register"]
pub mod comp1_icfr;
#[doc = "Comparator option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1_or](comp1_or) module"]
pub type COMP1_OR = crate::Reg<u32, _COMP1_OR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP1_OR;
#[doc = "`read()` method returns [comp1_or::R](comp1_or::R) reader structure"]
impl crate::Readable for COMP1_OR {}
#[doc = "`write(|w| ..)` method takes [comp1_or::W](comp1_or::W) writer structure"]
impl crate::Writable for COMP1_OR {}
#[doc = "Comparator option register"]
pub mod comp1_or;
#[doc = "Comparator configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1_cfgr1](comp1_cfgr1) module"]
pub type COMP1_CFGR1 = crate::Reg<u32, _COMP1_CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP1_CFGR1;
#[doc = "`read()` method returns [comp1_cfgr1::R](comp1_cfgr1::R) reader structure"]
impl crate::Readable for COMP1_CFGR1 {}
#[doc = "`write(|w| ..)` method takes [comp1_cfgr1::W](comp1_cfgr1::W) writer structure"]
impl crate::Writable for COMP1_CFGR1 {}
#[doc = "Comparator configuration register 1"]
pub mod comp1_cfgr1;
#[doc = "Comparator configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1_cfgr2](comp1_cfgr2) module"]
pub type COMP1_CFGR2 = crate::Reg<u32, _COMP1_CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP1_CFGR2;
#[doc = "`read()` method returns [comp1_cfgr2::R](comp1_cfgr2::R) reader structure"]
impl crate::Readable for COMP1_CFGR2 {}
#[doc = "`write(|w| ..)` method takes [comp1_cfgr2::W](comp1_cfgr2::W) writer structure"]
impl crate::Writable for COMP1_CFGR2 {}
#[doc = "Comparator configuration register 2"]
pub mod comp1_cfgr2;
