#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TZSC control register"]
    pub tzsc_cr: TZSC_CR,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - TZSC security configuration register"]
    pub tzsc_seccfgr1: TZSC_SECCFGR1,
    _reserved2: [u8; 12usize],
    #[doc = "0x20 - TZSC privilege configuration register 1"]
    pub tzsc_privcfgr1: TZSC_PRIVCFGR1,
    _reserved3: [u8; 268usize],
    #[doc = "0x130 - Unprivileged Water Mark 1 register"]
    pub tzsc_mpcwm1_upwmr: TZSC_MPCWM1_UPWMR,
    #[doc = "0x134 - Unprivileged Writable Water Mark 1 register"]
    pub tzsc_mpcwm1_upwwmr: TZSC_MPCWM1_UPWWMR,
    #[doc = "0x138 - Unprivileged Water Mark 2 register"]
    pub tzsc_mpcwm2_upwmr: TZSC_MPCWM2_UPWMR,
    _reserved6: [u8; 4usize],
    #[doc = "0x140 - Unprivileged Water Mark 3 register"]
    pub tzsc_mpcwm3_upwmr: TZSC_MPCWM3_UPWMR,
}
#[doc = "TZSC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzsc_cr](tzsc_cr) module"]
pub type TZSC_CR = crate::Reg<u32, _TZSC_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZSC_CR;
#[doc = "`read()` method returns [tzsc_cr::R](tzsc_cr::R) reader structure"]
impl crate::Readable for TZSC_CR {}
#[doc = "`write(|w| ..)` method takes [tzsc_cr::W](tzsc_cr::W) writer structure"]
impl crate::Writable for TZSC_CR {}
#[doc = "TZSC control register"]
pub mod tzsc_cr;
#[doc = "TZSC security configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzsc_seccfgr1](tzsc_seccfgr1) module"]
pub type TZSC_SECCFGR1 = crate::Reg<u32, _TZSC_SECCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZSC_SECCFGR1;
#[doc = "`read()` method returns [tzsc_seccfgr1::R](tzsc_seccfgr1::R) reader structure"]
impl crate::Readable for TZSC_SECCFGR1 {}
#[doc = "`write(|w| ..)` method takes [tzsc_seccfgr1::W](tzsc_seccfgr1::W) writer structure"]
impl crate::Writable for TZSC_SECCFGR1 {}
#[doc = "TZSC security configuration register"]
pub mod tzsc_seccfgr1;
#[doc = "TZSC privilege configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzsc_privcfgr1](tzsc_privcfgr1) module"]
pub type TZSC_PRIVCFGR1 = crate::Reg<u32, _TZSC_PRIVCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZSC_PRIVCFGR1;
#[doc = "`read()` method returns [tzsc_privcfgr1::R](tzsc_privcfgr1::R) reader structure"]
impl crate::Readable for TZSC_PRIVCFGR1 {}
#[doc = "`write(|w| ..)` method takes [tzsc_privcfgr1::W](tzsc_privcfgr1::W) writer structure"]
impl crate::Writable for TZSC_PRIVCFGR1 {}
#[doc = "TZSC privilege configuration register 1"]
pub mod tzsc_privcfgr1;
#[doc = "Unprivileged Water Mark 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzsc_mpcwm1_upwmr](tzsc_mpcwm1_upwmr) module"]
pub type TZSC_MPCWM1_UPWMR = crate::Reg<u32, _TZSC_MPCWM1_UPWMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZSC_MPCWM1_UPWMR;
#[doc = "`read()` method returns [tzsc_mpcwm1_upwmr::R](tzsc_mpcwm1_upwmr::R) reader structure"]
impl crate::Readable for TZSC_MPCWM1_UPWMR {}
#[doc = "`write(|w| ..)` method takes [tzsc_mpcwm1_upwmr::W](tzsc_mpcwm1_upwmr::W) writer structure"]
impl crate::Writable for TZSC_MPCWM1_UPWMR {}
#[doc = "Unprivileged Water Mark 1 register"]
pub mod tzsc_mpcwm1_upwmr;
#[doc = "Unprivileged Writable Water Mark 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzsc_mpcwm1_upwwmr](tzsc_mpcwm1_upwwmr) module"]
pub type TZSC_MPCWM1_UPWWMR = crate::Reg<u32, _TZSC_MPCWM1_UPWWMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZSC_MPCWM1_UPWWMR;
#[doc = "`read()` method returns [tzsc_mpcwm1_upwwmr::R](tzsc_mpcwm1_upwwmr::R) reader structure"]
impl crate::Readable for TZSC_MPCWM1_UPWWMR {}
#[doc = "`write(|w| ..)` method takes [tzsc_mpcwm1_upwwmr::W](tzsc_mpcwm1_upwwmr::W) writer structure"]
impl crate::Writable for TZSC_MPCWM1_UPWWMR {}
#[doc = "Unprivileged Writable Water Mark 1 register"]
pub mod tzsc_mpcwm1_upwwmr;
#[doc = "Unprivileged Water Mark 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzsc_mpcwm2_upwmr](tzsc_mpcwm2_upwmr) module"]
pub type TZSC_MPCWM2_UPWMR = crate::Reg<u32, _TZSC_MPCWM2_UPWMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZSC_MPCWM2_UPWMR;
#[doc = "`read()` method returns [tzsc_mpcwm2_upwmr::R](tzsc_mpcwm2_upwmr::R) reader structure"]
impl crate::Readable for TZSC_MPCWM2_UPWMR {}
#[doc = "`write(|w| ..)` method takes [tzsc_mpcwm2_upwmr::W](tzsc_mpcwm2_upwmr::W) writer structure"]
impl crate::Writable for TZSC_MPCWM2_UPWMR {}
#[doc = "Unprivileged Water Mark 2 register"]
pub mod tzsc_mpcwm2_upwmr;
#[doc = "Unprivileged Water Mark 3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzsc_mpcwm3_upwmr](tzsc_mpcwm3_upwmr) module"]
pub type TZSC_MPCWM3_UPWMR = crate::Reg<u32, _TZSC_MPCWM3_UPWMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZSC_MPCWM3_UPWMR;
#[doc = "`read()` method returns [tzsc_mpcwm3_upwmr::R](tzsc_mpcwm3_upwmr::R) reader structure"]
impl crate::Readable for TZSC_MPCWM3_UPWMR {}
#[doc = "`write(|w| ..)` method takes [tzsc_mpcwm3_upwmr::W](tzsc_mpcwm3_upwmr::W) writer structure"]
impl crate::Writable for TZSC_MPCWM3_UPWMR {}
#[doc = "Unprivileged Water Mark 3 register"]
pub mod tzsc_mpcwm3_upwmr;
