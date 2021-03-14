#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ICACHE control register"]
    pub icache_cr: ICACHE_CR,
    #[doc = "0x04 - ICACHE status register"]
    pub icache_sr: ICACHE_SR,
    #[doc = "0x08 - ICACHE interrupt enable register"]
    pub icache_ier: ICACHE_IER,
    #[doc = "0x0c - ICACHE flag clear register"]
    pub icache_fcr: ICACHE_FCR,
    #[doc = "0x10 - ICACHE hit monitor register"]
    pub icache_hmonr: ICACHE_HMONR,
    #[doc = "0x14 - ICACHE miss monitor register"]
    pub icache_mmonr: ICACHE_MMONR,
    _reserved6: [u8; 8usize],
    #[doc = "0x20 - ICACHE region configuration register"]
    pub icache_crr0: ICACHE_CRR0,
    #[doc = "0x24 - ICACHE region configuration register"]
    pub icache_crr1: ICACHE_CRR1,
    #[doc = "0x28 - ICACHE region configuration register"]
    pub icache_crr2: ICACHE_CRR2,
    #[doc = "0x2c - ICACHE region configuration register"]
    pub icache_crr3: ICACHE_CRR3,
}
#[doc = "ICACHE control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_cr](icache_cr) module"]
pub type ICACHE_CR = crate::Reg<u32, _ICACHE_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICACHE_CR;
#[doc = "`read()` method returns [icache_cr::R](icache_cr::R) reader structure"]
impl crate::Readable for ICACHE_CR {}
#[doc = "`write(|w| ..)` method takes [icache_cr::W](icache_cr::W) writer structure"]
impl crate::Writable for ICACHE_CR {}
#[doc = "ICACHE control register"]
pub mod icache_cr;
#[doc = "ICACHE status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_sr](icache_sr) module"]
pub type ICACHE_SR = crate::Reg<u32, _ICACHE_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICACHE_SR;
#[doc = "`read()` method returns [icache_sr::R](icache_sr::R) reader structure"]
impl crate::Readable for ICACHE_SR {}
#[doc = "ICACHE status register"]
pub mod icache_sr;
#[doc = "ICACHE interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_ier](icache_ier) module"]
pub type ICACHE_IER = crate::Reg<u32, _ICACHE_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICACHE_IER;
#[doc = "`read()` method returns [icache_ier::R](icache_ier::R) reader structure"]
impl crate::Readable for ICACHE_IER {}
#[doc = "`write(|w| ..)` method takes [icache_ier::W](icache_ier::W) writer structure"]
impl crate::Writable for ICACHE_IER {}
#[doc = "ICACHE interrupt enable register"]
pub mod icache_ier;
#[doc = "ICACHE flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_fcr](icache_fcr) module"]
pub type ICACHE_FCR = crate::Reg<u32, _ICACHE_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICACHE_FCR;
#[doc = "`write(|w| ..)` method takes [icache_fcr::W](icache_fcr::W) writer structure"]
impl crate::Writable for ICACHE_FCR {}
#[doc = "ICACHE flag clear register"]
pub mod icache_fcr;
#[doc = "ICACHE hit monitor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_hmonr](icache_hmonr) module"]
pub type ICACHE_HMONR = crate::Reg<u32, _ICACHE_HMONR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICACHE_HMONR;
#[doc = "`read()` method returns [icache_hmonr::R](icache_hmonr::R) reader structure"]
impl crate::Readable for ICACHE_HMONR {}
#[doc = "ICACHE hit monitor register"]
pub mod icache_hmonr;
#[doc = "ICACHE miss monitor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_mmonr](icache_mmonr) module"]
pub type ICACHE_MMONR = crate::Reg<u32, _ICACHE_MMONR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICACHE_MMONR;
#[doc = "`read()` method returns [icache_mmonr::R](icache_mmonr::R) reader structure"]
impl crate::Readable for ICACHE_MMONR {}
#[doc = "ICACHE miss monitor register"]
pub mod icache_mmonr;
#[doc = "ICACHE region configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_crr0](icache_crr0) module"]
pub type ICACHE_CRR0 = crate::Reg<u32, _ICACHE_CRR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICACHE_CRR0;
#[doc = "`read()` method returns [icache_crr0::R](icache_crr0::R) reader structure"]
impl crate::Readable for ICACHE_CRR0 {}
#[doc = "`write(|w| ..)` method takes [icache_crr0::W](icache_crr0::W) writer structure"]
impl crate::Writable for ICACHE_CRR0 {}
#[doc = "ICACHE region configuration register"]
pub mod icache_crr0;
#[doc = "ICACHE region configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_crr1](icache_crr1) module"]
pub type ICACHE_CRR1 = crate::Reg<u32, _ICACHE_CRR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICACHE_CRR1;
#[doc = "`read()` method returns [icache_crr1::R](icache_crr1::R) reader structure"]
impl crate::Readable for ICACHE_CRR1 {}
#[doc = "`write(|w| ..)` method takes [icache_crr1::W](icache_crr1::W) writer structure"]
impl crate::Writable for ICACHE_CRR1 {}
#[doc = "ICACHE region configuration register"]
pub mod icache_crr1;
#[doc = "ICACHE region configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_crr2](icache_crr2) module"]
pub type ICACHE_CRR2 = crate::Reg<u32, _ICACHE_CRR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICACHE_CRR2;
#[doc = "`read()` method returns [icache_crr2::R](icache_crr2::R) reader structure"]
impl crate::Readable for ICACHE_CRR2 {}
#[doc = "`write(|w| ..)` method takes [icache_crr2::W](icache_crr2::W) writer structure"]
impl crate::Writable for ICACHE_CRR2 {}
#[doc = "ICACHE region configuration register"]
pub mod icache_crr2;
#[doc = "ICACHE region configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_crr3](icache_crr3) module"]
pub type ICACHE_CRR3 = crate::Reg<u32, _ICACHE_CRR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICACHE_CRR3;
#[doc = "`read()` method returns [icache_crr3::R](icache_crr3::R) reader structure"]
impl crate::Readable for ICACHE_CRR3 {}
#[doc = "`write(|w| ..)` method takes [icache_crr3::W](icache_crr3::W) writer structure"]
impl crate::Writable for ICACHE_CRR3 {}
#[doc = "ICACHE region configuration register"]
pub mod icache_crr3;
