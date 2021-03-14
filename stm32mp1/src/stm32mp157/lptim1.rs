#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LPTIM interrupt and status register"]
    pub lptim_isr: LPTIM_ISR,
    #[doc = "0x04 - LPTIM interrupt clear register"]
    pub lptim_icr: LPTIM_ICR,
    #[doc = "0x08 - LPTIM interrupt enable register"]
    pub lptim_ier: LPTIM_IER,
    #[doc = "0x0c - LPTIM configuration register"]
    pub lptim_cfgr: LPTIM_CFGR,
    #[doc = "0x10 - LPTIM control register"]
    pub lptim_cr: LPTIM_CR,
    #[doc = "0x14 - LPTIM compare register"]
    pub lptim_cmp: LPTIM_CMP,
    #[doc = "0x18 - LPTIM autoreload register"]
    pub lptim_arr: LPTIM_ARR,
    #[doc = "0x1c - LPTIM counter register"]
    pub lptim_cnt: LPTIM_CNT,
    _reserved8: [u8; 4usize],
    #[doc = "0x24 - LPTIM configuration register 2"]
    pub lptim_cfgr2: LPTIM_CFGR2,
    _reserved9: [u8; 968usize],
    #[doc = "0x3f0 - LPTIM 1 peripheral hardware configuration register"]
    pub lptim1_hwcfgr: LPTIM1_HWCFGR,
    #[doc = "0x3f4 - LPTIM peripheral version identification register"]
    pub lptim_verr: LPTIM_VERR,
    #[doc = "0x3f8 - LPTIM peripheral type identification register"]
    pub lptim_pidr: LPTIM_PIDR,
    #[doc = "0x3fc - LPTIM registers map size identification register"]
    pub lptim_sidr: LPTIM_SIDR,
}
#[doc = "LPTIM interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_isr](lptim_isr) module"]
pub type LPTIM_ISR = crate::Reg<u32, _LPTIM_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM_ISR;
#[doc = "`read()` method returns [lptim_isr::R](lptim_isr::R) reader structure"]
impl crate::Readable for LPTIM_ISR {}
#[doc = "LPTIM interrupt and status register"]
pub mod lptim_isr;
#[doc = "LPTIM interrupt clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_icr](lptim_icr) module"]
pub type LPTIM_ICR = crate::Reg<u32, _LPTIM_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM_ICR;
#[doc = "`write(|w| ..)` method takes [lptim_icr::W](lptim_icr::W) writer structure"]
impl crate::Writable for LPTIM_ICR {}
#[doc = "LPTIM interrupt clear register"]
pub mod lptim_icr;
#[doc = "LPTIM interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_ier](lptim_ier) module"]
pub type LPTIM_IER = crate::Reg<u32, _LPTIM_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM_IER;
#[doc = "`read()` method returns [lptim_ier::R](lptim_ier::R) reader structure"]
impl crate::Readable for LPTIM_IER {}
#[doc = "`write(|w| ..)` method takes [lptim_ier::W](lptim_ier::W) writer structure"]
impl crate::Writable for LPTIM_IER {}
#[doc = "LPTIM interrupt enable register"]
pub mod lptim_ier;
#[doc = "LPTIM configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_cfgr](lptim_cfgr) module"]
pub type LPTIM_CFGR = crate::Reg<u32, _LPTIM_CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM_CFGR;
#[doc = "`read()` method returns [lptim_cfgr::R](lptim_cfgr::R) reader structure"]
impl crate::Readable for LPTIM_CFGR {}
#[doc = "`write(|w| ..)` method takes [lptim_cfgr::W](lptim_cfgr::W) writer structure"]
impl crate::Writable for LPTIM_CFGR {}
#[doc = "LPTIM configuration register"]
pub mod lptim_cfgr;
#[doc = "LPTIM control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_cr](lptim_cr) module"]
pub type LPTIM_CR = crate::Reg<u32, _LPTIM_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM_CR;
#[doc = "`read()` method returns [lptim_cr::R](lptim_cr::R) reader structure"]
impl crate::Readable for LPTIM_CR {}
#[doc = "`write(|w| ..)` method takes [lptim_cr::W](lptim_cr::W) writer structure"]
impl crate::Writable for LPTIM_CR {}
#[doc = "LPTIM control register"]
pub mod lptim_cr;
#[doc = "LPTIM compare register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_cmp](lptim_cmp) module"]
pub type LPTIM_CMP = crate::Reg<u32, _LPTIM_CMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM_CMP;
#[doc = "`read()` method returns [lptim_cmp::R](lptim_cmp::R) reader structure"]
impl crate::Readable for LPTIM_CMP {}
#[doc = "`write(|w| ..)` method takes [lptim_cmp::W](lptim_cmp::W) writer structure"]
impl crate::Writable for LPTIM_CMP {}
#[doc = "LPTIM compare register"]
pub mod lptim_cmp;
#[doc = "LPTIM autoreload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_arr](lptim_arr) module"]
pub type LPTIM_ARR = crate::Reg<u32, _LPTIM_ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM_ARR;
#[doc = "`read()` method returns [lptim_arr::R](lptim_arr::R) reader structure"]
impl crate::Readable for LPTIM_ARR {}
#[doc = "`write(|w| ..)` method takes [lptim_arr::W](lptim_arr::W) writer structure"]
impl crate::Writable for LPTIM_ARR {}
#[doc = "LPTIM autoreload register"]
pub mod lptim_arr;
#[doc = "LPTIM counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_cnt](lptim_cnt) module"]
pub type LPTIM_CNT = crate::Reg<u32, _LPTIM_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM_CNT;
#[doc = "`read()` method returns [lptim_cnt::R](lptim_cnt::R) reader structure"]
impl crate::Readable for LPTIM_CNT {}
#[doc = "LPTIM counter register"]
pub mod lptim_cnt;
#[doc = "LPTIM configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_cfgr2](lptim_cfgr2) module"]
pub type LPTIM_CFGR2 = crate::Reg<u32, _LPTIM_CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM_CFGR2;
#[doc = "`read()` method returns [lptim_cfgr2::R](lptim_cfgr2::R) reader structure"]
impl crate::Readable for LPTIM_CFGR2 {}
#[doc = "`write(|w| ..)` method takes [lptim_cfgr2::W](lptim_cfgr2::W) writer structure"]
impl crate::Writable for LPTIM_CFGR2 {}
#[doc = "LPTIM configuration register 2"]
pub mod lptim_cfgr2;
#[doc = "LPTIM 1 peripheral hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim1_hwcfgr](lptim1_hwcfgr) module"]
pub type LPTIM1_HWCFGR = crate::Reg<u32, _LPTIM1_HWCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM1_HWCFGR;
#[doc = "`read()` method returns [lptim1_hwcfgr::R](lptim1_hwcfgr::R) reader structure"]
impl crate::Readable for LPTIM1_HWCFGR {}
#[doc = "LPTIM 1 peripheral hardware configuration register"]
pub mod lptim1_hwcfgr;
#[doc = "LPTIM peripheral version identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_verr](lptim_verr) module"]
pub type LPTIM_VERR = crate::Reg<u32, _LPTIM_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM_VERR;
#[doc = "`read()` method returns [lptim_verr::R](lptim_verr::R) reader structure"]
impl crate::Readable for LPTIM_VERR {}
#[doc = "LPTIM peripheral version identification register"]
pub mod lptim_verr;
#[doc = "LPTIM peripheral type identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_pidr](lptim_pidr) module"]
pub type LPTIM_PIDR = crate::Reg<u32, _LPTIM_PIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM_PIDR;
#[doc = "`read()` method returns [lptim_pidr::R](lptim_pidr::R) reader structure"]
impl crate::Readable for LPTIM_PIDR {}
#[doc = "LPTIM peripheral type identification register"]
pub mod lptim_pidr;
#[doc = "LPTIM registers map size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_sidr](lptim_sidr) module"]
pub type LPTIM_SIDR = crate::Reg<u32, _LPTIM_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPTIM_SIDR;
#[doc = "`read()` method returns [lptim_sidr::R](lptim_sidr::R) reader structure"]
impl crate::Readable for LPTIM_SIDR {}
#[doc = "LPTIM registers map size identification register"]
pub mod lptim_sidr;
