#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TZSC control register"]
    pub tzsc_cr: TZSC_CR,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - TZSC secure configuration register 1"]
    pub tzsc_seccfgr1: TZSC_SECCFGR1,
    #[doc = "0x14 - TZSC secure configuration register 2"]
    pub tzsc_seccfgr2: TZSC_SECCFGR2,
    _reserved3: [u8; 8usize],
    #[doc = "0x20 - TZSC privilege configuration register 1"]
    pub tzsc_privcfgr1: TZSC_PRIVCFGR1,
    #[doc = "0x24 - TZSC privilege configuration register 2"]
    pub tzsc_privcfgr2: TZSC_PRIVCFGR2,
    _reserved5: [u8; 8usize],
    #[doc = "0x30 - TZSC external memory non-secure watermark register 1"]
    pub tzsc_mpcwm1_nswmr1: TZSC_MPCWM1_NSWMR1,
    #[doc = "0x34 - TZSC external memory non-secure watermark register 1"]
    pub tzsc_mpcwm1_nswmr2: TZSC_MPCWM1_NSWMR2,
    #[doc = "0x38 - TZSC external memory non-secure watermark register 1"]
    pub tzsc_mpcwm2_nswmr1: TZSC_MPCWM2_NSWMR1,
    #[doc = "0x3c - TZSC external memory non-secure watermark register 2"]
    pub tzsc_mpcwm2_nswmr2: TZSC_MPCWM2_NSWMR2,
    #[doc = "0x40 - TZSC external memory non-secure watermark register 2"]
    pub tzsc_mpcwm3_nswmr1: TZSC_MPCWM3_NSWMR1,
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
#[doc = "TZSC secure configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzsc_seccfgr1](tzsc_seccfgr1) module"]
pub type TZSC_SECCFGR1 = crate::Reg<u32, _TZSC_SECCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZSC_SECCFGR1;
#[doc = "`read()` method returns [tzsc_seccfgr1::R](tzsc_seccfgr1::R) reader structure"]
impl crate::Readable for TZSC_SECCFGR1 {}
#[doc = "`write(|w| ..)` method takes [tzsc_seccfgr1::W](tzsc_seccfgr1::W) writer structure"]
impl crate::Writable for TZSC_SECCFGR1 {}
#[doc = "TZSC secure configuration register 1"]
pub mod tzsc_seccfgr1;
#[doc = "TZSC secure configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzsc_seccfgr2](tzsc_seccfgr2) module"]
pub type TZSC_SECCFGR2 = crate::Reg<u32, _TZSC_SECCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZSC_SECCFGR2;
#[doc = "`read()` method returns [tzsc_seccfgr2::R](tzsc_seccfgr2::R) reader structure"]
impl crate::Readable for TZSC_SECCFGR2 {}
#[doc = "`write(|w| ..)` method takes [tzsc_seccfgr2::W](tzsc_seccfgr2::W) writer structure"]
impl crate::Writable for TZSC_SECCFGR2 {}
#[doc = "TZSC secure configuration register 2"]
pub mod tzsc_seccfgr2;
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
#[doc = "TZSC privilege configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzsc_privcfgr2](tzsc_privcfgr2) module"]
pub type TZSC_PRIVCFGR2 = crate::Reg<u32, _TZSC_PRIVCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZSC_PRIVCFGR2;
#[doc = "`read()` method returns [tzsc_privcfgr2::R](tzsc_privcfgr2::R) reader structure"]
impl crate::Readable for TZSC_PRIVCFGR2 {}
#[doc = "`write(|w| ..)` method takes [tzsc_privcfgr2::W](tzsc_privcfgr2::W) writer structure"]
impl crate::Writable for TZSC_PRIVCFGR2 {}
#[doc = "TZSC privilege configuration register 2"]
pub mod tzsc_privcfgr2;
#[doc = "TZSC external memory non-secure watermark register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzsc_mpcwm1_nswmr1](tzsc_mpcwm1_nswmr1) module"]
pub type TZSC_MPCWM1_NSWMR1 = crate::Reg<u32, _TZSC_MPCWM1_NSWMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZSC_MPCWM1_NSWMR1;
#[doc = "`read()` method returns [tzsc_mpcwm1_nswmr1::R](tzsc_mpcwm1_nswmr1::R) reader structure"]
impl crate::Readable for TZSC_MPCWM1_NSWMR1 {}
#[doc = "`write(|w| ..)` method takes [tzsc_mpcwm1_nswmr1::W](tzsc_mpcwm1_nswmr1::W) writer structure"]
impl crate::Writable for TZSC_MPCWM1_NSWMR1 {}
#[doc = "TZSC external memory non-secure watermark register 1"]
pub mod tzsc_mpcwm1_nswmr1;
#[doc = "TZSC external memory non-secure watermark register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzsc_mpcwm1_nswmr2](tzsc_mpcwm1_nswmr2) module"]
pub type TZSC_MPCWM1_NSWMR2 = crate::Reg<u32, _TZSC_MPCWM1_NSWMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZSC_MPCWM1_NSWMR2;
#[doc = "`read()` method returns [tzsc_mpcwm1_nswmr2::R](tzsc_mpcwm1_nswmr2::R) reader structure"]
impl crate::Readable for TZSC_MPCWM1_NSWMR2 {}
#[doc = "`write(|w| ..)` method takes [tzsc_mpcwm1_nswmr2::W](tzsc_mpcwm1_nswmr2::W) writer structure"]
impl crate::Writable for TZSC_MPCWM1_NSWMR2 {}
#[doc = "TZSC external memory non-secure watermark register 1"]
pub mod tzsc_mpcwm1_nswmr2;
#[doc = "TZSC external memory non-secure watermark register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzsc_mpcwm2_nswmr1](tzsc_mpcwm2_nswmr1) module"]
pub type TZSC_MPCWM2_NSWMR1 = crate::Reg<u32, _TZSC_MPCWM2_NSWMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZSC_MPCWM2_NSWMR1;
#[doc = "`read()` method returns [tzsc_mpcwm2_nswmr1::R](tzsc_mpcwm2_nswmr1::R) reader structure"]
impl crate::Readable for TZSC_MPCWM2_NSWMR1 {}
#[doc = "`write(|w| ..)` method takes [tzsc_mpcwm2_nswmr1::W](tzsc_mpcwm2_nswmr1::W) writer structure"]
impl crate::Writable for TZSC_MPCWM2_NSWMR1 {}
#[doc = "TZSC external memory non-secure watermark register 1"]
pub mod tzsc_mpcwm2_nswmr1;
#[doc = "TZSC external memory non-secure watermark register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzsc_mpcwm3_nswmr1](tzsc_mpcwm3_nswmr1) module"]
pub type TZSC_MPCWM3_NSWMR1 = crate::Reg<u32, _TZSC_MPCWM3_NSWMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZSC_MPCWM3_NSWMR1;
#[doc = "`read()` method returns [tzsc_mpcwm3_nswmr1::R](tzsc_mpcwm3_nswmr1::R) reader structure"]
impl crate::Readable for TZSC_MPCWM3_NSWMR1 {}
#[doc = "`write(|w| ..)` method takes [tzsc_mpcwm3_nswmr1::W](tzsc_mpcwm3_nswmr1::W) writer structure"]
impl crate::Writable for TZSC_MPCWM3_NSWMR1 {}
#[doc = "TZSC external memory non-secure watermark register 2"]
pub mod tzsc_mpcwm3_nswmr1;
#[doc = "TZSC external memory non-secure watermark register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzsc_mpcwm2_nswmr2](tzsc_mpcwm2_nswmr2) module"]
pub type TZSC_MPCWM2_NSWMR2 = crate::Reg<u32, _TZSC_MPCWM2_NSWMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZSC_MPCWM2_NSWMR2;
#[doc = "`read()` method returns [tzsc_mpcwm2_nswmr2::R](tzsc_mpcwm2_nswmr2::R) reader structure"]
impl crate::Readable for TZSC_MPCWM2_NSWMR2 {}
#[doc = "`write(|w| ..)` method takes [tzsc_mpcwm2_nswmr2::W](tzsc_mpcwm2_nswmr2::W) writer structure"]
impl crate::Writable for TZSC_MPCWM2_NSWMR2 {}
#[doc = "TZSC external memory non-secure watermark register 2"]
pub mod tzsc_mpcwm2_nswmr2;
