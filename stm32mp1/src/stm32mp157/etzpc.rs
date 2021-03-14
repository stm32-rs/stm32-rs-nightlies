#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ETZPC ROM secure size definition"]
    pub etzpc_tzma0_size: ETZPC_TZMA0_SIZE,
    #[doc = "0x04 - ETZPC RAM secure size definition"]
    pub etzpc_tzma1_size: ETZPC_TZMA1_SIZE,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Register reset values"]
    pub etzpc_decprot0: ETZPC_DECPROT0,
    #[doc = "0x14 - Register reset values"]
    pub etzpc_decprot1: ETZPC_DECPROT1,
    #[doc = "0x18 - Register reset values"]
    pub etzpc_decprot2: ETZPC_DECPROT2,
    #[doc = "0x1c - Register reset values"]
    pub etzpc_decprot3: ETZPC_DECPROT3,
    #[doc = "0x20 - Register reset values"]
    pub etzpc_decprot4: ETZPC_DECPROT4,
    #[doc = "0x24 - Register reset values"]
    pub etzpc_decprot5: ETZPC_DECPROT5,
    _reserved8: [u8; 8usize],
    #[doc = "0x30 - ETZPC decprot lock 0 register"]
    pub etzpc_decprot_lock0: ETZPC_DECPROT_LOCK0,
    #[doc = "0x34 - ETZPC decprot lock 1 register"]
    pub etzpc_decprot_lock1: ETZPC_DECPROT_LOCK1,
    #[doc = "0x38 - ETZPC decprot lock 2 register"]
    pub etzpc_decprot_lock2: ETZPC_DECPROT_LOCK2,
    _reserved11: [u8; 948usize],
    #[doc = "0x3f0 - ETZPC IP HW configuration register"]
    pub etzpc_hwcfgr: ETZPC_HWCFGR,
    #[doc = "0x3f4 - ETZPC IP version register"]
    pub etzpc_verr: ETZPC_VERR,
    #[doc = "0x3f8 - ETZPC IP version register"]
    pub etzpc_idr: ETZPC_IDR,
    #[doc = "0x3fc - ETZPC IP version register"]
    pub etzpc_sidr: ETZPC_SIDR,
}
#[doc = "ETZPC ROM secure size definition\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_tzma0_size](etzpc_tzma0_size) module"]
pub type ETZPC_TZMA0_SIZE = crate::Reg<u32, _ETZPC_TZMA0_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_TZMA0_SIZE;
#[doc = "`read()` method returns [etzpc_tzma0_size::R](etzpc_tzma0_size::R) reader structure"]
impl crate::Readable for ETZPC_TZMA0_SIZE {}
#[doc = "`write(|w| ..)` method takes [etzpc_tzma0_size::W](etzpc_tzma0_size::W) writer structure"]
impl crate::Writable for ETZPC_TZMA0_SIZE {}
#[doc = "ETZPC ROM secure size definition"]
pub mod etzpc_tzma0_size;
#[doc = "ETZPC RAM secure size definition\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_tzma1_size](etzpc_tzma1_size) module"]
pub type ETZPC_TZMA1_SIZE = crate::Reg<u32, _ETZPC_TZMA1_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_TZMA1_SIZE;
#[doc = "`read()` method returns [etzpc_tzma1_size::R](etzpc_tzma1_size::R) reader structure"]
impl crate::Readable for ETZPC_TZMA1_SIZE {}
#[doc = "`write(|w| ..)` method takes [etzpc_tzma1_size::W](etzpc_tzma1_size::W) writer structure"]
impl crate::Writable for ETZPC_TZMA1_SIZE {}
#[doc = "ETZPC RAM secure size definition"]
pub mod etzpc_tzma1_size;
#[doc = "Register reset values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_decprot0](etzpc_decprot0) module"]
pub type ETZPC_DECPROT0 = crate::Reg<u32, _ETZPC_DECPROT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_DECPROT0;
#[doc = "`read()` method returns [etzpc_decprot0::R](etzpc_decprot0::R) reader structure"]
impl crate::Readable for ETZPC_DECPROT0 {}
#[doc = "`write(|w| ..)` method takes [etzpc_decprot0::W](etzpc_decprot0::W) writer structure"]
impl crate::Writable for ETZPC_DECPROT0 {}
#[doc = "Register reset values"]
pub mod etzpc_decprot0;
#[doc = "Register reset values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_decprot1](etzpc_decprot1) module"]
pub type ETZPC_DECPROT1 = crate::Reg<u32, _ETZPC_DECPROT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_DECPROT1;
#[doc = "`read()` method returns [etzpc_decprot1::R](etzpc_decprot1::R) reader structure"]
impl crate::Readable for ETZPC_DECPROT1 {}
#[doc = "`write(|w| ..)` method takes [etzpc_decprot1::W](etzpc_decprot1::W) writer structure"]
impl crate::Writable for ETZPC_DECPROT1 {}
#[doc = "Register reset values"]
pub mod etzpc_decprot1;
#[doc = "Register reset values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_decprot2](etzpc_decprot2) module"]
pub type ETZPC_DECPROT2 = crate::Reg<u32, _ETZPC_DECPROT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_DECPROT2;
#[doc = "`read()` method returns [etzpc_decprot2::R](etzpc_decprot2::R) reader structure"]
impl crate::Readable for ETZPC_DECPROT2 {}
#[doc = "`write(|w| ..)` method takes [etzpc_decprot2::W](etzpc_decprot2::W) writer structure"]
impl crate::Writable for ETZPC_DECPROT2 {}
#[doc = "Register reset values"]
pub mod etzpc_decprot2;
#[doc = "Register reset values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_decprot3](etzpc_decprot3) module"]
pub type ETZPC_DECPROT3 = crate::Reg<u32, _ETZPC_DECPROT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_DECPROT3;
#[doc = "`read()` method returns [etzpc_decprot3::R](etzpc_decprot3::R) reader structure"]
impl crate::Readable for ETZPC_DECPROT3 {}
#[doc = "`write(|w| ..)` method takes [etzpc_decprot3::W](etzpc_decprot3::W) writer structure"]
impl crate::Writable for ETZPC_DECPROT3 {}
#[doc = "Register reset values"]
pub mod etzpc_decprot3;
#[doc = "Register reset values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_decprot4](etzpc_decprot4) module"]
pub type ETZPC_DECPROT4 = crate::Reg<u32, _ETZPC_DECPROT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_DECPROT4;
#[doc = "`read()` method returns [etzpc_decprot4::R](etzpc_decprot4::R) reader structure"]
impl crate::Readable for ETZPC_DECPROT4 {}
#[doc = "`write(|w| ..)` method takes [etzpc_decprot4::W](etzpc_decprot4::W) writer structure"]
impl crate::Writable for ETZPC_DECPROT4 {}
#[doc = "Register reset values"]
pub mod etzpc_decprot4;
#[doc = "Register reset values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_decprot5](etzpc_decprot5) module"]
pub type ETZPC_DECPROT5 = crate::Reg<u32, _ETZPC_DECPROT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_DECPROT5;
#[doc = "`read()` method returns [etzpc_decprot5::R](etzpc_decprot5::R) reader structure"]
impl crate::Readable for ETZPC_DECPROT5 {}
#[doc = "`write(|w| ..)` method takes [etzpc_decprot5::W](etzpc_decprot5::W) writer structure"]
impl crate::Writable for ETZPC_DECPROT5 {}
#[doc = "Register reset values"]
pub mod etzpc_decprot5;
#[doc = "ETZPC decprot lock 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_decprot_lock0](etzpc_decprot_lock0) module"]
pub type ETZPC_DECPROT_LOCK0 = crate::Reg<u32, _ETZPC_DECPROT_LOCK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_DECPROT_LOCK0;
#[doc = "`read()` method returns [etzpc_decprot_lock0::R](etzpc_decprot_lock0::R) reader structure"]
impl crate::Readable for ETZPC_DECPROT_LOCK0 {}
#[doc = "`write(|w| ..)` method takes [etzpc_decprot_lock0::W](etzpc_decprot_lock0::W) writer structure"]
impl crate::Writable for ETZPC_DECPROT_LOCK0 {}
#[doc = "ETZPC decprot lock 0 register"]
pub mod etzpc_decprot_lock0;
#[doc = "ETZPC decprot lock 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_decprot_lock1](etzpc_decprot_lock1) module"]
pub type ETZPC_DECPROT_LOCK1 = crate::Reg<u32, _ETZPC_DECPROT_LOCK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_DECPROT_LOCK1;
#[doc = "`read()` method returns [etzpc_decprot_lock1::R](etzpc_decprot_lock1::R) reader structure"]
impl crate::Readable for ETZPC_DECPROT_LOCK1 {}
#[doc = "`write(|w| ..)` method takes [etzpc_decprot_lock1::W](etzpc_decprot_lock1::W) writer structure"]
impl crate::Writable for ETZPC_DECPROT_LOCK1 {}
#[doc = "ETZPC decprot lock 1 register"]
pub mod etzpc_decprot_lock1;
#[doc = "ETZPC decprot lock 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_decprot_lock2](etzpc_decprot_lock2) module"]
pub type ETZPC_DECPROT_LOCK2 = crate::Reg<u32, _ETZPC_DECPROT_LOCK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_DECPROT_LOCK2;
#[doc = "`read()` method returns [etzpc_decprot_lock2::R](etzpc_decprot_lock2::R) reader structure"]
impl crate::Readable for ETZPC_DECPROT_LOCK2 {}
#[doc = "`write(|w| ..)` method takes [etzpc_decprot_lock2::W](etzpc_decprot_lock2::W) writer structure"]
impl crate::Writable for ETZPC_DECPROT_LOCK2 {}
#[doc = "ETZPC decprot lock 2 register"]
pub mod etzpc_decprot_lock2;
#[doc = "ETZPC IP HW configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_hwcfgr](etzpc_hwcfgr) module"]
pub type ETZPC_HWCFGR = crate::Reg<u32, _ETZPC_HWCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_HWCFGR;
#[doc = "`read()` method returns [etzpc_hwcfgr::R](etzpc_hwcfgr::R) reader structure"]
impl crate::Readable for ETZPC_HWCFGR {}
#[doc = "ETZPC IP HW configuration register"]
pub mod etzpc_hwcfgr;
#[doc = "ETZPC IP version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_verr](etzpc_verr) module"]
pub type ETZPC_VERR = crate::Reg<u32, _ETZPC_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_VERR;
#[doc = "`read()` method returns [etzpc_verr::R](etzpc_verr::R) reader structure"]
impl crate::Readable for ETZPC_VERR {}
#[doc = "ETZPC IP version register"]
pub mod etzpc_verr;
#[doc = "ETZPC IP version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_idr](etzpc_idr) module"]
pub type ETZPC_IDR = crate::Reg<u32, _ETZPC_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_IDR;
#[doc = "`read()` method returns [etzpc_idr::R](etzpc_idr::R) reader structure"]
impl crate::Readable for ETZPC_IDR {}
#[doc = "ETZPC IP version register"]
pub mod etzpc_idr;
#[doc = "ETZPC IP version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_sidr](etzpc_sidr) module"]
pub type ETZPC_SIDR = crate::Reg<u32, _ETZPC_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_SIDR;
#[doc = "`read()` method returns [etzpc_sidr::R](etzpc_sidr::R) reader structure"]
impl crate::Readable for ETZPC_SIDR {}
#[doc = "ETZPC IP version register"]
pub mod etzpc_sidr;
